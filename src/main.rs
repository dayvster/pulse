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
    } else {
        println!("You need to specify either a pid or a name.");
        println!("Seek help with --help or -h");
    }
}
