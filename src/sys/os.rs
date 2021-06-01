use std::{process::Command, str};

use regex::Regex;

pub struct OS {
    name: String,
    kernel: String,
    uptime: String,
}

impl OS {
    pub fn get_info() -> OS {
        match cfg!(windows) {
            true => {
                let name_regex = Regex::new(r"(?<=\s\s)(\w+\s?)+").unwrap();
                let name_cmd = Command::new("cmd")
                    .args(vec![
                        "/C",
                        "systeminfo | findstr /B /C:'OS Name'"
                    ])
                    .output()
                    .unwrap();
                let mut name = str::from_utf8(&name_cmd.stdout)
                    .unwrap();
                name = name_regex.captures(&name)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str();
                let mut boot_time_cmd = Command::new("cmd")
                    .args(vec![
                        "/C",
                        "systeminfo | find 'System Boot Time'"
                    ])
                    .output()
                    .unwrap();

                OS {
                    name: name.to_string(),
                    kernel: String::from("Windows NT kernel"),
                    uptime: String::from(""),
                }
            },
            false => {
                OS {
                    name: String::from(""),
                    kernel: String::from(""),
                    uptime: String::from(""),
                }
            }
        }
    }
}
