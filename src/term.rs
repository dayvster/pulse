use std::{
    io::{stdin, stdout, Read, Stdin, StdinLock, Stdout, Write},
    os::fd::{AsFd, AsRawFd},
};

use clap::Parser;
use termion::{
    clear, color,
    cursor::{self, DetectCursorPos},
    raw::IntoRawMode,
};

pub struct Term {
    stdout: Stdout,
    stdin: Stdin,
}

impl Term {
    pub fn new() -> Self {
        Self {
            stdout: stdout(),
            stdin: stdin(),
        }
    }

    pub fn term_flush(&self) {
        match self.stdout_raw().flush() {
            Ok(_) => {}
            Err(_) => {
                println!("Error flushing stdout.");
                std::process::exit(1);
            }
        }
    }

    pub fn write_all(&self, output: &str) {
        match self.stdout_raw().write_all(output.as_bytes()) {
            Ok(_) => {}
            Err(_) => {
                println!("Error writing to stdout.");
                std::process::exit(1);
            }
        }
    }

    /**
        ## Print Output
        Prints the output to the terminal.
    */
    pub fn print_output(&self, pid: u32, name: &str, cpu: f64, mem: f64, cpu_total: f64) {
        // let cpu = format!("{:.2}", cpu);
        let cpu = {
            match cpu {
                _ if cpu < 10.0 => format!("{:.2}", cpu),
                _ if cpu < 100.0 => format!("{:.1}", cpu),
                _ => format!("{:.0}", cpu),
            }
        };
        let mem = format!("{:.2}", mem);
        let cpu_total = format!("{:.2}", cpu_total);
        let pid = format!("{}", pid);
        let name = format!("{}", name);

        let cursor_pos: (u16, u16) = {
            match self.stdout_raw().cursor_pos() {
                Ok(pos) => pos,
                Err(_) => {
                    println!("Error getting cursor position.");
                    std::process::exit(1);
                }
            }
        };

        let output = format!(
            "{}{}PID: {} {}NAME: {} {}CPU {}% {}MEM: {}Mb {}CPU Total: {} {}",
            cursor::Goto(1, cursor_pos.1),
            clear::CurrentLine,
            pid,
            color::Fg(color::Reset),
            name,
            color::Fg(color::Blue),
            cpu,
            color::Fg(color::Reset),
            mem,
            color::Fg(color::Yellow),
            cpu_total,
            color::Fg(color::Reset)
        );
        self.write_all(&output);
        self.term_flush();
    }

    fn stdout_raw(&self) -> termion::raw::RawTerminal<std::io::StdoutLock> {
        match self.stdout.lock().into_raw_mode() {
            Ok(stdout) => stdout,
            Err(_) => {
                println!("Error getting stdout.");
                std::process::exit(1);
            }
        }
    }
}
/**
   ## Args
    The command line arguments for the program.
*/
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
}
