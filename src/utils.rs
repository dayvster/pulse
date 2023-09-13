use psutil;
use psutil::process::ProcessCollector;
pub struct Utils {
    collector: ProcessCollector,
}

impl Utils {
    pub fn new() -> Self {
        let collector = ProcessCollector::new().unwrap();
        Self { collector }
    }
    pub fn get_pid(&self, name: &str) -> u32 {
        let mut pid = None;
        for p in self.collector.processes.iter() {
            if p.1.name().unwrap() == name {
                pid = Some(p.1.pid())
            }
        }
        if pid.is_none() {
            println!("No process found with name: {}", name);
            std::process::exit(1);
        }
        pid.unwrap()
    }

    pub fn get_name(&self, pid: &u32) -> String {
        let mut name = None;
        for p in self.collector.processes.iter() {
            if p.1.pid() == *pid {
                name = Some(p.1.name().unwrap())
            }
        }
        if name.is_none() {
            println!("No process found with pid: {}", pid);
            std::process::exit(1);
        }
        name.unwrap()
    }
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
    pub fn get_cpu(&self, pid: &u32) -> Option<f64> {
        let mut cpu: Option<f64> = None;

        self.collector.processes.iter().for_each(|p| {
            if p.1.pid() == *pid {
                match p.1.clone().cpu_percent() {
                    Ok(cpu_percent) => {
                        cpu = Some(cpu_percent.clone() as f64);
                    }
                    Err(_) => {
                        cpu = Some(0.0);
                    }
                }
            }
        });
        cpu
    }
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
}
