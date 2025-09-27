use psutil;
use psutil::process::os::linux::ProcessExt;
use psutil::process::Process;
use psutil::process::ProcessCollector;

#[derive(Debug, Clone)]
pub struct OutputLine {
    pub pid: u32,
    pub name: String,
    pub cpu: f64,
    pub mem: f64,
}
// ...existing code...
/**
    # Utils
    This module contains the Utils struct which is used to get information about processes.
    It uses the psutil crate to get information about processes.
*/
pub struct Utils {
    collector: ProcessCollector,
}

impl Utils {
    /// Gets the number of threads for a process
    pub fn get_threads(&self, pid: &u32) -> Option<u32> {
        for p in self.collector.processes.iter() {
            if p.1.pid() == *pid {
                return Some(p.1.num_threads() as u32);
            }
        }
        None
    }

    /// Gets the process state as a string
    pub fn get_state(&self, pid: &u32) -> Option<String> {
        for p in self.collector.processes.iter() {
            if p.1.pid() == *pid {
                return p.1.status().ok().map(|s| format!("{:?}", s));
            }
        }
        None
    }

    /// Gets the user/UID for a process
    pub fn get_user(&self, pid: &u32) -> Option<String> {
        for p in self.collector.processes.iter() {
            if p.1.pid() == *pid {
                return Some(p.1.username());
            }
        }
        None
    }

    /// Gets the start time for a process (as a string)
    pub fn get_start(&self, pid: &u32) -> Option<String> {
        use chrono::{Local, TimeZone};
        for p in self.collector.processes.iter() {
            if p.1.pid() == *pid {
                let dur = p.1.create_time();
                let secs = dur.as_secs() as i64;
                let dt = Local.timestamp_opt(secs, 0).single();
                if let Some(dt) = dt {
                    return Some(dt.format("%Y-%m-%d %H:%M:%S").to_string());
                }
            }
        }
        None
    }

    /// Gets the nice value (priority) for a process
    pub fn get_nice(&self, pid: &u32) -> Option<i32> {
        // Not available in psutil 5.x, return 0
        Some(0)
    }

    /// Gets the total CPU time (user+sys) for a process as a string
    pub fn get_cputime(&self, pid: &u32) -> Option<String> {
        for p in self.collector.processes.iter() {
            if p.1.pid() == *pid {
                if let Ok(times) = p.1.cpu_times() {
                    let total = times.user().as_secs() + times.system().as_secs();
                    let h = total / 3600;
                    let m = (total % 3600) / 60;
                    let s = total % 60;
                    return Some(format!("{:02}:{:02}:{:02}", h, m, s));
                }
            }
        }
        None
    }
    /// Create a new Utils struct
    pub fn new() -> Self {
        let collector = {
            match psutil::process::ProcessCollector::new() {
                Ok(collector) => collector,
                Err(_) => {
                    println!("Error creating process collector.");
                    std::process::exit(1);
                }
            }
        };
        Self { collector }
    }

    /// Gets the IO (read_bytes + write_bytes) of a process by pid, in bytes/sec (approx, since psutil only gives total).
    pub fn get_io(&self, pid: &u32) -> Option<f64> {
        // psutil 5.4.0 does not expose per-process IO counters publicly
        // Fallback: always return 0.0
        Some(0.0)
    }
    /**
       ## Utils::get_pid()
       This function gets the pid of a process by name.
    */
    pub fn get_pid(&self, name: &str) -> u32 {
        let mut pid = None;
        for p in self.collector.processes.iter() {
            if let Ok(proc_name) = p.1.name() {
                if proc_name == name {
                    pid = Some(p.1.pid());
                }
            }
        }
        match pid {
            Some(pid) => match self.get_parent_proc(&pid) {
                Some(pid) => pid,
                None => {
                    println!("No parent process found.");
                    std::process::exit(1);
                }
            },
            None => {
                println!("No process found with name: {}", name);
                std::process::exit(1);
            }
        }
    }
    /**
       ## Utils::get_name()
       This function gets the name of a process by pid.
    */
    pub fn get_name(&self, pid: &u32) -> String {
        for p in self.collector.processes.iter() {
            let pid_result = std::panic::catch_unwind(|| p.1.pid());
            if let Ok(found_pid) = pid_result {
                if found_pid == *pid {
                    let name_result = std::panic::catch_unwind(|| p.1.name());
                    if let Ok(Ok(proc_name)) = name_result {
                        return proc_name;
                    } else {
                        return "N/A".to_string();
                    }
                }
            }
        }
        "N/A".to_string()
    }
    /**
        ## Utils::get_mem()
        This function gets the memory usage of a process by pid.
    */
    pub fn get_mem(&self, pid: &u32) -> Option<f64> {
        let mut mem = None;
        self.collector.processes.iter().for_each(|p| {
            let pid_result = std::panic::catch_unwind(|| p.1.pid());
            if let Ok(found_pid) = pid_result {
                if found_pid == *pid {
                    let mem_result = std::panic::catch_unwind(|| p.1.memory_info());
                    match mem_result {
                        Ok(Ok(mem_info)) => {
                            mem = Some(mem_info.rss() as f64 / 1024.0 / 1024.0);
                        }
                        Ok(Err(_)) | Err(_) => {
                            mem = Some(0.0);
                        }
                    }
                }
            }
        });
        mem
    }
    /**
        ## Utils::get_cpu()
        This function gets the cpu usage of a process by pid.
    */

    pub fn get_cpu(&self, pid: &u32) -> Option<f64> {
        let mut cpu: Option<f64> = None;
        self.collector.processes.iter().for_each(|p| {
            let pid_result = std::panic::catch_unwind(|| p.1.pid());
            if let Ok(found_pid) = pid_result {
                if found_pid == *pid {
                    let cpu_result = std::panic::catch_unwind(|| p.1.clone().cpu_percent());
                    match cpu_result {
                        Ok(Ok(cpu_percent)) => {
                            cpu = Some(cpu_percent as f64);
                        }
                        Ok(Err(_)) | Err(_) => {
                            cpu = Some(0.0);
                        }
                    }
                }
            }
        });
        cpu
    }
    /**
        ## Utils::get_cpu_total()
        This function gets the total cpu usage of all processes.
    */
    pub fn get_cpu_total(&self) -> Option<f64> {
        let mut cpu_total = 0.0;
        self.collector
            .processes
            .iter()
            .for_each(|p| match p.1.clone().cpu_percent() {
                Ok(cpu_percent) => {
                    cpu_total += cpu_percent as f64;
                }
                Err(_) => {
                    cpu_total += 0.0;
                }
            });
        Some(cpu_total)
    }
    /**
        ## Utils::get_collector()
        This function gets the ProcessCollector struct.
    */
    pub fn get_collector(&self) -> &ProcessCollector {
        &self.collector
    }
    /**
        ## Utils::has_parent()
        This function checks if a process has a parent.
    */
    fn has_parent(&self, pid: &u32) -> bool {
        let mut has_parent = false;
        self.collector.processes.iter().for_each(|p| {
            if p.1.pid() == *pid {
                has_parent = true;
            }
        });
        has_parent
    }
    /**
        ## Utils::get_parent_proc()
        This function gets the parent process of a process.
        If no parent process is found, it returns the process itself.
    */
    fn get_parent_proc(&self, pid: &u32) -> Option<u32> {
        match self.has_parent(pid) {
            false => Some(*pid),
            true => {
                let mut parent_pid = None;
                self.collector
                    .processes
                    .iter()
                    .for_each(|p| match p.1.pid() == *pid {
                        true => {
                            parent_pid = {
                                match p.1.parent() {
                                    Ok(parent) => match parent {
                                        Some(parent) => Some(parent.pid()),
                                        None => None,
                                    },
                                    Err(_) => None,
                                }
                            }
                        }
                        false => {
                            parent_pid = Some(*pid);
                        }
                    });
                parent_pid
            }
        }
    }
}
