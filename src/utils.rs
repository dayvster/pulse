use psutil;
use psutil::process::ProcessCollector;
/**
    # Utils
    This module contains the Utils struct which is used to get information about processes.
    It uses the psutil crate to get information about processes.
*/
pub struct Utils {
    collector: ProcessCollector,
}

impl Utils {
    /**
       ## Utils::new()
       This function creates a new Utils struct.
    */
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
    /**
       ## Utils::get_pid()
       This function gets the pid of a process by name.
    */
    pub fn get_pid(&self, name: &str) -> u32 {
        let mut pid = None;

        for p in self.collector.processes.iter() {
            match p.1.name() {
                Ok(proc_name) => {
                    if proc_name == name {
                        pid = Some(p.1.pid());
                    }
                }
                Err(_) => {
                    println!("Error getting process name.");
                    std::process::exit(1);
                }
            }
        }
        let pid = {
            match pid {
                Some(pid) => pid,
                None => {
                    println!("No process found with name: {}", name);
                    std::process::exit(1);
                }
            }
        };
        match self.get_parent_proc(&pid) {
            Some(pid) => pid,
            None => {
                println!("No parent process found.");
                std::process::exit(1);
            }
        }
    }
    /**
       ## Utils::get_name()
       This function gets the name of a process by pid.
    */
    pub fn get_name(&self, pid: &u32) -> String {
        let mut name = None;
        for p in self.collector.processes.iter() {
            match p.1.name() {
                Ok(proc_name) => {
                    if p.1.pid() == *pid {
                        name = Some(proc_name);
                    }
                }
                Err(_) => {
                    println!("Error getting process name.");
                    std::process::exit(1);
                }
            }
        }
        if name.is_none() {
            println!("No process found with pid: {}", pid);
            std::process::exit(1);
        }
        match name {
            Some(name) => name,
            None => {
                println!("No process found with pid: {}", pid);
                std::process::exit(1);
            }
        }
    }
    /**
        ## Utils::get_mem()
        This function gets the memory usage of a process by pid.
    */
    pub fn get_mem(&self, pid: &u32) -> Option<f64> {
        let mut mem = None;
        self.collector.processes.iter().for_each(|p| {
            if p.1.pid() == *pid {
                match p.1.memory_info() {
                    Ok(mem_info) => {
                        mem = Some(mem_info.rss() as f64 / 1024.0 / 1024.0);
                    }
                    Err(_) => {
                        mem = Some(0.0);
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
            if p.1.pid() == *pid {
                match p.1.clone().cpu_percent() {
                    Ok(cpu_percent) => {
                        cpu = Some(cpu_percent as f64);
                    }
                    Err(_) => {
                        cpu = Some(0.0);
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
