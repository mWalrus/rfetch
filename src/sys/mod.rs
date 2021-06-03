use crate::colored::*;
use std::fmt;

pub mod hw;
pub mod os;

pub struct SysInfo {
    pub os: String,
    pub kernel: String,
    pub uptime: String,
    pub terminal: String,
    pub shell: String,
    pub cpu: String,
    pub gpu: String,
    pub memory: String,
    pub hostname: String,
}

impl fmt::Display for SysInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}\n{} {}",
            "OS:".bold().blue(), self.os,
            "Kernel:".bold().blue(), self.kernel,
            "Uptime:".bold().blue(), self.uptime,
            "Terminal:".bold().blue(), self.terminal,
            "Shell:".bold().blue(), self.shell,
            "CPU:".bold().blue(), self.cpu,
            "GPU:".bold().blue(), self.gpu,
            "Memory:".bold().blue(), self.memory,
            "Hostname:".bold().blue(), self.hostname
        )
    }
}

impl SysInfo {
    pub fn new() -> SysInfo {
        let cpu = hw::cpu_info();
        let gpu = hw::gpu_info();
        let os = os::os_name();
        let uptime = os::uptime();

        SysInfo {
            os,
            kernel: String::from("value"),
            uptime,
            terminal: String::from("value"),
            shell: String::from("value"),
            cpu,
            gpu,
            memory: String::from("value"),
            hostname: String::from("value"),
        }
    }
}
