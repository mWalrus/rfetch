use std::{fmt, process::Command, str};

use regex::Regex;

pub struct GPU {
    vendor: String,
    model: String,
}

impl fmt::Display for GPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.vendor, self.model)
    }
}

impl GPU {
    pub fn get_info() -> GPU {
        match cfg!(windows) {
            true => {
                let gpu_cmd = Command::new("cmd")
                    .args(vec![
                        "/C",
                        "wimc path win32_videocardcontroller get description"
                    ])
                    .output()
                    .unwrap();
                let (vendor, model) = str::from_utf8(&gpu_cmd.stdout)
                    .unwrap()
                    .split_once(" ")
                    .unwrap();
                GPU {
                    vendor: vendor.to_string(),
                    model: model.to_string(),
                }
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

                GPU {
                    vendor,
                    model,
                }
            }
        }
    }
}

pub struct CPU {
    pub name: String,
    pub cores: u16,
    pub mhz: f32,
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) @ {}MHz", self.name, self.cores, self.mhz)
    }
}

impl CPU {
    pub fn get_info() -> CPU {
        let legal_regex = Regex::new(r"\((TM|tm|R|r)\)").unwrap();
        let name_end_regex = Regex::new(r"\s@\s\d+(\.\d+)?(MHz|GHz)").unwrap();

        // windows: wmic cpu get name, maxclockspeed, numberofcores
        // linux:
        //      mhz = sudo dmesg | grep "MHz" | head -1 | awk '{print $5}'
        //      cores = grep -m1 "^siblings" /proc/cpuinfo
        //
        match cfg!(windows) {
            true => {
                // run command and get output bytes
                let wimc = Command::new("cmd")
                .args(vec![
                    "/C",
                    "wimc cpu get name, maxclockspeed, numberofcores"
                ])
                .output()
                .expect("Failed to execute wimc");
                // convert command output bytes from utf8 to human readable string
                let wimc_out = str::from_utf8(&wimc.stdout).unwrap();

                // format the outputted data
                let raw_cpu_data: &str = wimc_out.split("\n").collect::<Vec<_>>()[2];
                let white_space_regex = Regex::new(r"\s{2,}").unwrap();
                let formatted_output = white_space_regex.replace_all(raw_cpu_data, ",");
                let cpu_info: Vec<_> = formatted_output.split(",").filter(|n| !n.is_empty()).collect();

                // break out info from the data
                let mhz = cpu_info[0].parse::<f32>().unwrap();
                let mut name = name_end_regex.replace(cpu_info[1], "").to_string();
                name = legal_regex.replace_all(&name, "").to_string();
                let cores = cpu_info[2].parse::<u16>().unwrap();

                CPU {
                    name,
                    cores,
                    mhz
                }
            },
            false => {
                let lscpu_output = Command::new("bash")
                    .args(vec![
                        "-c",
                        r"lscpu | grep -E '^Core\(s\) per socket|^Model name|^CPU MHz' | sed 's/^.*:\s*//'"
                    ])
                    .output()
                    .unwrap();
                let items = str::from_utf8(&lscpu_output.stdout)
                    .unwrap()
                    .split("\n")
                    .collect::<Vec<_>>();

                let cores = items[0].parse::<u16>().unwrap();
                let mut name = legal_regex.replace(items[1], "").to_string();
                name = name_end_regex.replace(&name, "").to_string();
                let mhz = items[2].parse::<f32>().unwrap();
                CPU {
                    name,
                    cores,
                    mhz,
                }
            }
        }
    }
}
