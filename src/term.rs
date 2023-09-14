use std::io::{stdout, Write};

use clap::Parser;
use termion::{
    clear, color,
    cursor::{self, DetectCursorPos},
    raw::IntoRawMode,
};

pub struct Term {}

impl Term {
    pub fn new() -> Self {
        Self {}
    }
    pub fn print_output(&self, pid: u32, name: &str, cpu: f64, mem: f64, cpu_total: f64) {
        let cpu = format!("{:.2}", cpu);
        let mem = format!("{:.2}", mem);
        let cpu_total = format!("{:.2}", cpu_total);
        let pid = format!("{}", pid);
        let name = format!("{}", name);

        let mut stdout = stdout().into_raw_mode().unwrap();
        let cursor_pos = stdout.cursor_pos().unwrap();
        write!(stdout, "{} ", cursor::Goto(1, cursor_pos.1)).unwrap();
        write!(stdout, "{} ", clear::CurrentLine).unwrap();
        write!(stdout, "{} ", color::Fg(color::Green)).unwrap();
        write!(stdout, "PID: {} ", pid).unwrap();
        write!(stdout, "{} ", color::Fg(color::Reset)).unwrap();
        write!(stdout, "{} ", color::Fg(color::Blue)).unwrap();
        write!(stdout, "NAME: {} ", name).unwrap();
        write!(stdout, "{} ", color::Fg(color::Reset)).unwrap();
        write!(stdout, "{} ", color::Fg(color::Red)).unwrap();
        write!(stdout, "CPU {}% ", cpu).unwrap();
        write!(stdout, "{} ", color::Fg(color::Reset)).unwrap();
        write!(stdout, "{} ", color::Fg(color::Yellow)).unwrap();
        write!(stdout, "MEM: {}Mb ", mem).unwrap();
        write!(stdout, "{} ", color::Fg(color::Reset)).unwrap();
        write!(stdout, "{} ", color::Fg(color::Magenta)).unwrap();
        write!(stdout, "CPU Total: {} ", cpu_total).unwrap();
        write!(stdout, "{} ", color::Fg(color::Reset)).unwrap();
        stdout.flush().unwrap();
    }
}

#[derive(Parser, Debug)]
#[command(author, version)]
pub struct Args {
    #[arg(
        short,
        long,
        help = "The process ID of the process we wish to track.\n\
            EXAMPLE: 1234\n"
    )]
    pub pid: Option<String>,
    #[arg(short, long, help = "The name of the process to track.\n\
        EXAMPLE: firefox\n", 
        default_value =None)]
    pub name: Option<String>,
    #[arg(
        short,
        long,
        help = "The interval in seconds between each sample.\n\
        EXAMPLE: 1.5",
        default_value = "1.0"
    )]
    pub interval: Option<f64>,

    #[arg(short = 'p', short = 's', help = "Similar to the ps command.")]
    pub psmode: Option<bool>,
}
