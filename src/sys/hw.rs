use std::{fmt, process::Command, str};
use crate::colored::*;

use regex::Regex;

pub struct HW {
    pub cpu: String,
    pub gpu: String,
    pub mem: String,
}

impl fmt::Display for HW {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}\n{} {}\n{} {}",
            "CPU:".bold().blue(), &self.cpu,
            "GPU:".bold().blue(), &self.gpu,
            "MEM:".bold().blue(), &self.mem,
        )
    }
}

impl HW {
    pub fn new() -> HW {
        HW {
            cpu: cpu_info(),
            gpu: gpu_info(),
            mem: mem_info(),
        }
    }
}

pub fn gpu_info() -> String {
    let gpu_info;
    match cfg!(windows) {
        true => {
            let gpu_cmd = Command::new("cmd")
                .args(vec![
                    "/C",
                    "wmic path win32_VideoController get name /value"
                ])
                .output()
                .unwrap();
            gpu_info = str::from_utf8(&gpu_cmd.stdout)
                .unwrap()
                .replace("Name=", "")
                .replace("\n", "")
                .replace("\r", "");
        },
        false => {
            let vendor_cmd = Command::new("bash")
                .args(vec![
                    "-c",
                    r"lspci -vnn | grep VGA | sed 's/^.*[0-9]\]:\s//' | awk '{print $1}'"
                ])
                .output()
                .unwrap();
            let vendor = str::from_utf8(&vendor_cmd.stdout)
                .unwrap()
                .replace("\n", "");

            let model_cmd = Command::new("bash")
                .args(vec![
                    "-c",
                    r"lspci -vnn | grep VGA | sed 's/^.*[0-9]\]:\s//;s/\]\s//' | awk -F'[' '{print $2}'"
                ])
                .output()
                .unwrap();
            let model = str::from_utf8(&model_cmd.stdout)
                .unwrap()
                .replace("\n", "");

            gpu_info = format!("{} {}", vendor, model);
        }
    }
    gpu_info
}

pub fn cpu_info() -> String {
    let legal_regex = Regex::new(r"\((TM|tm|R|r)\)").unwrap();
    let name_end_regex = Regex::new(r"\s@\s\d+(\.\d+)?(MHz|GHz)").unwrap();

    // windows: wmic cpu get name, maxclockspeed, numberofcores
    // linux:
    //      mhz = sudo dmesg | grep "MHz" | head -1 | awk '{print $5}'
    //      cores = grep -m1 "^siblings" /proc/cpuinfo
    //
    let cpu_string;
    match cfg!(windows) {
        true => {
            // run command and get output bytes
            let wimc = Command::new("cmd")
            .args(vec![
                "/C",
                "wmic cpu get name, maxclockspeed, numberofcores"
            ])
            .output()
            .expect("Failed to execute wimc");
            // convert command output bytes from utf8 to human readable string
            let wimc_out = str::from_utf8(&wimc.stdout).unwrap();

            // format the outputted data
            let raw_cpu_data: &str = wimc_out.split("\n").collect::<Vec<_>>()[1];
            let white_space_regex = Regex::new(r"\s{2,}").unwrap();
            let formatted_output = white_space_regex.replace_all(raw_cpu_data, ",");
            let cpu_info: Vec<_> = formatted_output.split(",").filter(|n| !n.is_empty()).collect();

            // break out info from the data
            let mhz = cpu_info[0].parse::<f32>().unwrap();
            let mut name = name_end_regex.replace(cpu_info[1], "").to_string();
            name = legal_regex.replace_all(&name, "").to_string();
            let cores = cpu_info[2].parse::<u16>().unwrap();

            cpu_string = format!("{} ({}) @ {}MHz", &name, &cores, &mhz);
        },
        false => {
            let lscpu_output = Command::new("bash")
                .args(vec![
                    "-c",
                    r"lscpu | grep -E '^Core\(s\) per socket|^Model name|^CPU max MHz' | sed 's/^.*:\s*//'"
                ])
                .output()
                .unwrap();
            let items = str::from_utf8(&lscpu_output.stdout)
                .unwrap()
                .split("\n")
                .collect::<Vec<_>>();

            let cores = items[1].parse::<u16>().unwrap();
            let mut name = legal_regex.replace(items[0], "").to_string();
            name = name_end_regex.replace(&name, "").to_string();
            let mhz = items[2].parse::<f32>().unwrap();
            cpu_string = format!("{} ({}) @ {}MHz", &name, &cores, &mhz);
        }
    }
    cpu_string
}

fn mem_info() -> String {
    // We get the mem free and mem total and calculate from that
    // linux: grep 'Mem[^A]' /proc/meminfo | sed 's/\w*:\s*//; s/\skB//' | tr '\n' '&'
    // windows powershell: $ComputerMemory = Get-WmiObject -Class win32_operatingsystem -ErrorAction Stop; echo $ComputerMemory.TotalVisibleMemorySize $ComputerMemory.FreePhysicalMemory
    // 1 KB = 0.00095367431640625 MiB
    String::new()
}
