mod color;
mod tui;
mod user_utils;
mod utils;

use clap::Parser;
use clap::ValueEnum;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};

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

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum OutputMode {
    Pretty,
    Json,
    Csv,
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
    #[arg(long, default_value_t = false)]
    benchmark: bool,
    #[arg(long, default_value_t = 10.0)]
    duration: f64,
    #[arg(long, default_value_t = 1)]
    runs: usize,
    #[arg(long)]
    cmd: Option<String>,
    #[arg(long, value_enum, default_value_t = OutputMode::Pretty)]
    output: OutputMode,
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

    if args.benchmark {
        if let Some(cmd) = &args.cmd {
            // Benchmark command execution time and resource usage
            use std::process::{Command, Stdio};
            use std::thread::sleep;
            use std::time::{Duration, Instant};
            let utils = utils::Utils::new();
            let mut times = vec![];
            let mut max_cpus = vec![];
            let mut max_mems = vec![];
            for _ in 0..args.runs {
                let mut child = Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .expect("Failed to spawn command");
                let pid = child.id() as u32;
                let mut max_cpu = 0.0;
                let mut max_mem = 0.0;
                let poll_interval = Duration::from_millis(50);
                let start = Instant::now();
                loop {
                    // Check if process is still running
                    match child.try_wait() {
                        Ok(Some(_status)) => break,
                        Ok(None) => {
                            // Sample CPU and MEM
                            if let Some(cpu) = utils.get_cpu(&pid) {
                                if cpu > max_cpu {
                                    max_cpu = cpu;
                                }
                            }
                            if let Some(mem) = utils.get_mem(&pid) {
                                if mem > max_mem {
                                    max_mem = mem;
                                }
                            }
                            sleep(poll_interval);
                        }
                        Err(_) => break,
                    }
                }
                let elapsed = start.elapsed().as_secs_f64();
                times.push(elapsed);
                max_cpus.push(max_cpu);
                max_mems.push(max_mem);
            }
            let avg = times.iter().sum::<f64>() / times.len() as f64;
            let min = times.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let max_cpu = max_cpus.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let max_mem = max_mems.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            match args.output {
                OutputMode::Pretty => {
                    let mut table = Table::new();
                    table
                        .load_preset(UTF8_FULL)
                        .set_content_arrangement(ContentArrangement::Dynamic)
                        .set_header(vec![
                            Cell::new("Command").fg(Color::Cyan),
                            Cell::new("Runs").fg(Color::Cyan),
                            Cell::new("Avg (s)").fg(Color::Green),
                            Cell::new("Min (s)").fg(Color::Yellow),
                            Cell::new("Max (s)").fg(Color::Red),
                            Cell::new("Max CPU% ").fg(Color::Red),
                            Cell::new("Max RAM MB").fg(Color::Red),
                        ]);
                    table.add_row(vec![
                        Cell::new(cmd).fg(Color::White),
                        Cell::new(args.runs).fg(Color::White),
                        Cell::new(format!("{:.3}", avg)).fg(Color::Green),
                        Cell::new(format!("{:.3}", min)).fg(Color::Yellow),
                        Cell::new(format!("{:.3}", max)).fg(Color::Red),
                        Cell::new(format!("{:.2}", max_cpu)).fg(Color::Red),
                        Cell::new(format!("{:.2}", max_mem)).fg(Color::Red),
                    ]);
                    println!("\n{}", table);
                }
                OutputMode::Json => {
                    let obj = serde_json::json!({
                        "command": cmd,
                        "runs": args.runs,
                        "avg": avg,
                        "min": min,
                        "max": max,
                        "max_cpu": max_cpu,
                        "max_mem": max_mem
                    });
                    println!("{}", serde_json::to_string_pretty(&obj).unwrap());
                }
                OutputMode::Csv => {
                    println!("command,runs,avg,min,max,max_cpu,max_mem");
                    println!("{}", format!("{}", cmd.replace(",", " "))); // avoid CSV breakage
                    println!(
                        "{},{:.3},{:.3},{:.3},{:.2},{:.2}",
                        args.runs, avg, min, max, max_cpu, max_mem
                    );
                }
            }
        } else {
            // Benchmark process stats (per PID, no aggregation)
            let samples = (args.duration / args.interval).ceil() as usize;
            let mut cpu_samples = vec![vec![]; pids.len()];
            let mut mem_samples = vec![vec![]; pids.len()];
            for _ in 0..samples {
                for (i, pid) in pids.iter().enumerate() {
                    let cpu = std::panic::catch_unwind(|| utils.get_cpu(pid))
                        .ok()
                        .flatten()
                        .unwrap_or(0.0);
                    let mem = std::panic::catch_unwind(|| utils.get_mem(pid))
                        .ok()
                        .flatten()
                        .unwrap_or(0.0);
                    cpu_samples[i].push(cpu);
                    mem_samples[i].push(mem);
                }
                thread::sleep(time::Duration::from_secs_f64(args.interval));
            }
            match args.output {
                OutputMode::Pretty => {
                    let mut table = Table::new();
                    table
                        .load_preset(UTF8_FULL)
                        .set_content_arrangement(ContentArrangement::Dynamic)
                        .set_header(vec![
                            Cell::new("PID").fg(Color::Cyan),
                            Cell::new("NAME").fg(Color::Cyan),
                            Cell::new("Avg CPU% ").fg(Color::Green),
                            Cell::new("Max CPU% ").fg(Color::Red),
                            Cell::new("Avg RAM MB").fg(Color::Green),
                            Cell::new("Max RAM MB").fg(Color::Red),
                        ]);
                    for (i, pid) in pids.iter().enumerate() {
                        let name = utils.get_name(pid);
                        let avg_cpu =
                            cpu_samples[i].iter().sum::<f64>() / cpu_samples[i].len() as f64;
                        let max_cpu = cpu_samples[i]
                            .iter()
                            .cloned()
                            .fold(f64::NEG_INFINITY, f64::max);
                        let avg_mem =
                            mem_samples[i].iter().sum::<f64>() / mem_samples[i].len() as f64;
                        let max_mem = mem_samples[i]
                            .iter()
                            .cloned()
                            .fold(f64::NEG_INFINITY, f64::max);
                        table.add_row(vec![
                            Cell::new(pid).fg(Color::White),
                            Cell::new(truncate_name(&name, 18)).fg(Color::White),
                            Cell::new(format!("{:.2}", avg_cpu)).fg(Color::Green),
                            Cell::new(format!("{:.2}", max_cpu)).fg(Color::Red),
                            Cell::new(format!("{:.2}", avg_mem)).fg(Color::Green),
                            Cell::new(format!("{:.2}", max_mem)).fg(Color::Red),
                        ]);
                    }
                    println!("\n{}", table);
                }
                OutputMode::Json => {
                    let mut arr = vec![];
                    for (i, pid) in pids.iter().enumerate() {
                        let name = utils.get_name(pid);
                        let avg_cpu =
                            cpu_samples[i].iter().sum::<f64>() / cpu_samples[i].len() as f64;
                        let max_cpu = cpu_samples[i]
                            .iter()
                            .cloned()
                            .fold(f64::NEG_INFINITY, f64::max);
                        let avg_mem =
                            mem_samples[i].iter().sum::<f64>() / mem_samples[i].len() as f64;
                        let max_mem = mem_samples[i]
                            .iter()
                            .cloned()
                            .fold(f64::NEG_INFINITY, f64::max);
                        arr.push(serde_json::json!({
                            "pid": pid,
                            "name": name,
                            "avg_cpu": avg_cpu,
                            "max_cpu": max_cpu,
                            "avg_mem": avg_mem,
                            "max_mem": max_mem
                        }));
                    }
                    println!("{}", serde_json::to_string_pretty(&arr).unwrap());
                }
                OutputMode::Csv => {
                    println!("pid,name,avg_cpu,max_cpu,avg_mem,max_mem");
                    for (i, pid) in pids.iter().enumerate() {
                        let name = utils.get_name(pid).replace(",", " ");
                        let avg_cpu =
                            cpu_samples[i].iter().sum::<f64>() / cpu_samples[i].len() as f64;
                        let max_cpu = cpu_samples[i]
                            .iter()
                            .cloned()
                            .fold(f64::NEG_INFINITY, f64::max);
                        let avg_mem =
                            mem_samples[i].iter().sum::<f64>() / mem_samples[i].len() as f64;
                        let max_mem = mem_samples[i]
                            .iter()
                            .cloned()
                            .fold(f64::NEG_INFINITY, f64::max);
                        println!(
                            "{},{},{:.2},{:.2},{:.2},{:.2}",
                            pid, name, avg_cpu, max_cpu, avg_mem, max_mem
                        );
                    }
                }
            }
        }
    } else {
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
}
