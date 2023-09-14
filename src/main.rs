mod term;
mod utils;

use clap::Parser;

fn loop_output(pid: u32, name: &str, interval: f64, term: &term::Term, utils: &utils::Utils) {
    loop {
        let cpu = {
            match utils.get_cpu(&pid) {
                Some(cpu) => cpu,
                None => {
                    println!("No process found with pid: {}", pid);
                    std::process::exit(1);
                }
            }
        };
        let mem = {
            match utils.get_mem(&pid) {
                Some(mem) => mem,
                None => {
                    println!("No process found with pid: {}", pid);
                    std::process::exit(1);
                }
            }
        };
        let cpu_total = {
            match utils.get_cpu_total() {
                Some(cpu_total) => cpu_total,
                None => {
                    println!("No process found with pid: {}", pid);
                    std::process::exit(1);
                }
            }
        };
        term.print_output(pid, name, cpu, mem, cpu_total);
        std::thread::sleep(std::time::Duration::from_secs_f64(interval));
    }
}

fn main() {
    let args = term::Args::parse();
    let utils = utils::Utils::new();
    let term: term::Term = term::Term::new();

    match args.interval {
        Some(interval) => {
            if interval <= 0.0 {
                println!("Don't do that.");
                std::process::exit(1);
            }
        }
        None => {}
    }

    if args.name != None {
        let name = {
            match args.name {
                Some(name) => name,
                None => {
                    println!("You need to specify a name.");
                    std::process::exit(1);
                }
            }
        };

        let interval = {
            match args.interval {
                Some(interval) => interval,
                None => 1.0,
            }
        };

        let pid = utils.get_pid(&name);
        loop_output(pid, &name, interval, &term, &utils);
    } else if args.pid != None {
        let pid: u32 = {
            match args.pid {
                Some(pid) => match pid.parse::<u32>() {
                    Ok(pid) => pid,
                    Err(_) => {
                        println!("Invalid pid.");
                        std::process::exit(1);
                    }
                },
                None => {
                    println!("You need to specify a pid.");
                    std::process::exit(1);
                }
            }
        };

        let interval = {
            match args.interval {
                Some(interval) => interval,
                None => 1.0,
            }
        };
        loop_output(pid, &utils.get_name(&pid), interval, &term, &utils);
    } else {
        println!("You need to specify either a pid or a name.");
        println!("Seek help with --help or -h");
    }
}
