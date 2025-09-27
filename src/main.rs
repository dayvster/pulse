mod color;
mod tui;
mod user_utils;
mod utils;

use clap::Parser;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SortBy {
    Pid,
    Name,
    Cpu,
    Ram,
    RamPercent,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Order {
    Asc,
    Desc,
}

fn print_table(
    pids: &[u32],
    utils: &utils::Utils,
    sortby: SortBy,
    order: Order,
    limit: usize,
    show_io: bool,
) {
    use color::{GREEN, RED, RESET, YELLOW};
    if show_io {
        println!(
            "{: <6} {: <18} {: <8} {: <10} {: <8} {: <10}",
            "PID", "NAME", "CPU%", "RAM MB", "RAM %", "IO (B/s)"
        );
    } else {
        println!(
            "{: <6} {: <18} {: <8} {: <10} {: <8}",
            "PID", "NAME", "CPU%", "RAM MB", "RAM %"
        );
    }
    let total_mem = sys_info::mem_info()
        .map(|m| m.total as f64 / 1024.0)
        .unwrap_or(0.0); // in MB
    let mut rows = vec![];
    for pid in pids {
        let name =
            std::panic::catch_unwind(|| utils.get_name(pid)).unwrap_or_else(|_| "N/A".to_string());
        if name == "N/A" {
            continue;
        }
        let cpu = std::panic::catch_unwind(|| utils.get_cpu(pid))
            .ok()
            .flatten()
            .unwrap_or(0.0);
        let mem = std::panic::catch_unwind(|| utils.get_mem(pid))
            .ok()
            .flatten()
            .unwrap_or(0.0);
        let ram_percent = if total_mem > 0.0 {
            (mem / total_mem) * 100.0
        } else {
            0.0
        };
        let io = if show_io {
            std::panic::catch_unwind(|| utils.get_io(pid))
                .ok()
                .flatten()
                .unwrap_or(0.0)
        } else {
            0.0
        };
        rows.push((pid, name, cpu, mem, ram_percent, io));
    }

    // Sort (unchanged)
    match sortby {
        SortBy::Pid => {
            if order == Order::Asc {
                rows.sort_by_key(|r| *r.0);
            } else {
                rows.sort_by_key(|r| std::cmp::Reverse(*r.0));
            }
        }
        SortBy::Name => {
            if order == Order::Asc {
                rows.sort_by(|a, b| a.1.cmp(&b.1));
            } else {
                rows.sort_by(|a, b| b.1.cmp(&a.1));
            }
        }
        SortBy::Cpu => {
            if order == Order::Asc {
                rows.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
            } else {
                rows.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
            }
        }
        SortBy::Ram => {
            if order == Order::Asc {
                rows.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal));
            } else {
                rows.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));
            }
        }
        SortBy::RamPercent => {
            if order == Order::Asc {
                rows.sort_by(|a, b| a.4.partial_cmp(&b.4).unwrap_or(std::cmp::Ordering::Equal));
            } else {
                rows.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal));
            }
        }
    }

    for (pid, name, cpu, mem, ram_percent, io) in rows.into_iter().take(limit) {
        // Colorize CPU%
        let cpu_color = if cpu >= 50.0 {
            RED
        } else if cpu >= 20.0 {
            YELLOW
        } else {
            GREEN
        };
        // Colorize RAM (yellow if > 100MB)
        let mem_color = if mem > 100.0 { YELLOW } else { RESET };

        if show_io {
            println!(
                "{: <6} {: <18} {cpu_color}{: <8.1}{RESET} {mem_color}{: <10.1}{RESET} {: <8.1} {: <10.1}",
                pid,
                truncate_name(&name, 18),
                cpu,
                mem,
                ram_percent,
                io,
                cpu_color = cpu_color,
                mem_color = mem_color,
                RESET = RESET
            );
        } else {
            println!(
                "{: <6} {: <18} {cpu_color}{: <8.1}{RESET} {mem_color}{: <10.1}{RESET} {: <8.1}",
                pid,
                truncate_name(&name, 18),
                cpu,
                mem,
                ram_percent,
                cpu_color = cpu_color,
                mem_color = mem_color,
                RESET = RESET
            );
        }
    }
}

fn truncate_name(name: &str, max_len: usize) -> String {
    if name.chars().count() > max_len {
        let mut s = name.chars().take(max_len - 1).collect::<String>();
        s.push('…');
        s
    } else {
        name.to_string()
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    name: Vec<String>,
    #[arg(long)]
    pid: Vec<u32>,
    #[arg(short, long, default_value_t = 1.0)]
    interval: f64,
    #[arg(long, default_value_t = false)]
    no_interactive: bool,
    #[arg(short = 'A', long)]
    all: bool,
    #[arg(short = 'a', long)]
    current_user: bool,
    #[arg(long, value_enum, default_value_t = SortBy::Pid)]
    sortby: SortBy,
    #[arg(long, value_enum, default_value_t = Order::Asc)]
    order: Order,
    #[arg(long, default_value_t = 30)]
    limit: usize,
    #[arg(long, default_value_t = false)]
    io: bool,
}

use std::{thread, time};
fn main() {
    let utils = utils::Utils::new();
    let args = Args::parse();
    let all_pids: Vec<u32> = utils
        .get_collector()
        .processes
        .iter()
        .filter_map(|p| std::panic::catch_unwind(|| p.1.pid()).ok())
        .collect();

    use std::collections::HashSet;
    let mut pid_set = HashSet::new();

    // Add PIDs matching --name (if any)
    if !args.name.is_empty() {
        for pid in &all_pids {
            let proc_name = std::panic::catch_unwind(|| utils.get_name(pid))
                .unwrap_or_else(|_| "N/A".to_string())
                .to_lowercase();
            if args
                .name
                .iter()
                .any(|n| proc_name.contains(&n.to_lowercase()))
            {
                pid_set.insert(*pid);
            }
        }
    }

    // Add PIDs from --pid (if any)
    for pid in &args.pid {
        pid_set.insert(*pid);
    }

    // If neither --name nor --pid, show all
    let pids: Vec<u32> = if pid_set.is_empty() {
        all_pids
    } else {
        all_pids
            .into_iter()
            .filter(|pid| pid_set.contains(pid))
            .collect()
    };

    // Wait at least 0.5s to get meaningful CPU stats
    thread::sleep(time::Duration::from_millis(500));

    if args.no_interactive {
        print_table(&pids, &utils, args.sortby, args.order, args.limit, args.io);
    } else {
        // Interactive loop
        loop {
            // Clear screen (ANSI escape)
            print!("\x1b[2J\x1b[H");
            print_table(&pids, &utils, args.sortby, args.order, args.limit, args.io);
            // Sleep for 1s between updates
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
