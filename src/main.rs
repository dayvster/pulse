mod term;
mod utils;

use clap::Parser;

fn loop_output(pid: u32, name: &str, interval: f64, term: &term::Term, utils: &utils::Utils) {
    loop {
        let cpu = utils.get_cpu(&pid).unwrap();
        let mem = utils.get_mem(&pid).unwrap();
        let cpu_total = utils.get_cpu_total().unwrap();
        term.print_output(pid, name, cpu, mem, cpu_total);
        std::thread::sleep(std::time::Duration::from_secs_f64(interval));
    }
}

fn main() {
    let args = term::Args::parse();
    let utils = utils::Utils::new();
    let term: term::Term = term::Term::new();

    if args.name != None {
        let name = args.name.unwrap();
        let interval = args.interval.unwrap_or(1.0);

        let pid = utils.get_pid(&name);
        loop_output(pid, &name, interval, &term, &utils);
    } else if args.pid != None {
        let pid: u32 = args.pid.unwrap().parse::<u32>().unwrap();
        let interval = args.interval.unwrap_or(1.0);
        loop_output(pid, &utils.get_name(&pid), interval, &term, &utils);
    } else if args.psmode.unwrap() == true {
        // get top 6 processes by cpu usage just like ps
        let mut processes: Vec<(u32, String, f64)> = Vec::new();
        for p in utils.get_collector().processes.iter() {
            let pid = p.1.pid();
            let name = p.1.name().unwrap();
            let cpu = utils.get_cpu(&pid).unwrap();
            processes.push((pid, name, cpu));
        }
        processes.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        for p in processes.iter().take(6) {
            println!("{} {} {}", p.0, p.1, p.2);
        }
    } else {
        println!("You need to specify either a pid or a name.");
        println!("Seek help with --help or -h");
    }
}
