use std::{convert::TryFrom, fmt, fs::File, io::Read, process::Command, str};

use regex::Regex;

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
    pub fn get_cpu_info() -> CPU {
        let legal_regex = Regex::new(r"\((TM|tm|R|r)\)").unwrap();
        let name_end_regex = Regex::new(r"\s@\s\d+(\.\d+)?(MHz|GHz)").unwrap();

        // windows: wmic cpu get name, maxclockspeed, numberofcores
        // linux:
        //      mhz = sudo dmesg | grep "MHz" | head -1 | awk '{print $5}'
        //      cores = grep -m1 "^siblings" /proc/cpuinfo
        //
        match cfg!(windows) {
            true => {
                let (name, cores, mhz) = get_cpu_windows(legal_regex, name_end_regex);
                CPU {
                    name,
                    cores,
                    mhz
                }
            },
            false => {
                let (name, cores, mhz) = get_cpu_linux(legal_regex, name_end_regex);
                CPU {
                    name,
                    cores,
                    mhz,
                }
            }
        }
    }
}

fn get_cpu_windows(legal_regex: Regex, name_end_regex: Regex) -> (String, u16, f32) {
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
    (name, cores, mhz)
}

fn get_cpu_linux(legal_regex: Regex, name_end_regex: Regex) -> (String, u16, f32) {
    let mut buffer = String::new();
    match File::open("/proc/cpuinfo") {
        Ok(mut content) => content.read_to_string(&mut buffer).unwrap(),
        Err(e) => panic!("Failed to read file: {}", e),
    };
    // get file lines and retain the slice we need
    let file_lines: Vec<_> = buffer.split("\n")
        .collect::<Vec<&str>>()[..27]
        .to_vec();

    // filter for only the three parts we need
    let items = &file_lines
        .into_iter()
        .filter(|line| {
            line.contains("model name") ||
            line.contains("cpu MHz")    ||
            line.contains("cpu cores")
        })
        .map(|item| item.split("\t: ").collect::<Vec<_>>()[1])
        .collect::<Vec<_>>();

    let mut name = legal_regex.replace(items[0], "").to_string();
    name = name_end_regex.replace(&name, "").to_string();
    let cores = items[2].parse::<u16>().unwrap();
    let mhz = items[1].parse::<f32>().unwrap();
    (name, cores, mhz)
}
