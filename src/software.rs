use std::{fmt, process::Command, str};
use colored::Colorize;
use regex::Regex;
use std::collections::HashMap;

pub struct OS {
    name: String,
    kernel: String,
    uptime: String,
}

pub struct UserHost {
    user: String,
    hostname: String,
}

impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}\n{} {}\n{} {}",
            "OS:".bold().blue(), self.name,
            "Kernel:".bold().blue(), self.kernel,
            "Uptime:".bold().blue(), self.uptime,
        )
    }
}

impl OS {
    pub fn new() -> OS {
        OS {
            name: os_name(),
            kernel: kernel(),
            uptime: uptime()
        }
    }
}

impl fmt::Display for UserHost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}@{}\n{}",
            self.user.bold().blue(),
            self.hostname.bold().blue(),
            "-".repeat(self.user.len() + self.hostname.len() + 1)
        )
    }
}

impl UserHost {
    pub fn new() -> UserHost {
        let cmd = match cfg!(windows) {
            true => {
                Command::new("cmd")
                    .args(vec![
                        "/C",
                        "echo %username%@%computername%"
                    ])
                    .output()
                    .unwrap()
                        },
            false => {
                Command::new("bash")
                    .args(vec![
                        "-c",
                        "echo \"`whoami`@`cat /proc/sys/kernel/hostname`\""
                    ])
                    .output()
                    .unwrap()
            }
        };
        let mut output = str::from_utf8(&cmd.stdout)
            .unwrap()
            .splitn(2, '@');
        let user = output.next().unwrap().to_string();
        let hostname = output.next()
            .unwrap()
            .to_string()
            .replace("\n", "")
            .replace("\r", "");
        UserHost {
            user,
            hostname
        }
    }
}

pub fn os_name() -> String {
    match cfg!(windows) {
        true => {
            let name_cmd = Command::new("cmd")
                .args(vec![
                    "/C",
                    "wmic os get Caption /value"
                ])
                .output()
                .unwrap();
            let stdout = str::from_utf8(&name_cmd.stdout)
                .unwrap()
                .to_owned();
            let name = stdout
                .replace("Caption=", "")
                .replace("\n", "")
                .replace("\r", "");
            name.to_owned()
        },
        false => {
            let name_cmd = Command::new("bash")
                .args(vec![
                    "-c",
                    "grep '^NAME=' /etc/os-release | awk -F= '{print $2}' | sed 's/\"//g'"
                ])
                .output()
                .unwrap();
            let stdout = str::from_utf8(&name_cmd.stdout).unwrap();
            let name = &stdout.replace("\n", "");
            name.to_owned()
        }
    }
}

pub fn uptime() -> String {
    let replace_regex = Regex::new(r"\s{2,}:\s").unwrap();
    match cfg!(windows) {
        true => {
            let uptime_cmd = Command::new("powershell")
                .args(vec![
                    "-Command",
                    "(Get-Date) - (Get-CimInstance -ClassName Win32_OperatingSystem).LastBootUpTime"
                ])
                .output()
                .unwrap();
            let uptime_raw = str::from_utf8(&uptime_cmd.stdout)
                .unwrap()
                .to_owned();
            let uptime_split = uptime_raw.split("\n").collect::<Vec<_>>();
            let mut uptime_map = HashMap::new();
            for field in uptime_split[2..=5].into_iter() {
                let mut iter = replace_regex.splitn(&field, 2);
                let key = iter.next()
                    .unwrap()
                    .to_string();
                let value = iter.next()
                    .unwrap()
                    .replace("\r", "")
                    .parse::<i64>()
                    .unwrap();
                uptime_map.insert(key, value);
            }
            let mut output = String::new();
            for (key, value) in uptime_map.into_iter() {
                append_uptime_field(value, &mut output, &key);
            }
            output
        },
        false => {
            let uptime_cmd = Command::new("bash")
                .args(vec![
                    "-c",
                    "uptime -p | sed 's/up //'"
                ])
                .output()
                .unwrap();
            let stdout = str::from_utf8(&uptime_cmd.stdout).unwrap();
            let uptime = stdout.replace("\n", "");
            uptime
        }
    }
}

pub fn kernel() -> String {
    match cfg!(windows) {
        true => {
            let kernel_command = Command::new("cmd")
                .args(vec![
                    "/C",
                    "echo %os%"
                ])
                .output()
                .unwrap();
            str::from_utf8(&kernel_command.stdout)
                .unwrap()
                .replace("\n", "")
        },
        false => {
            let kernel_command = Command::new("bash")
                .args(vec![
                    "-c",
                    "uname -r"
                ])
                .output()
                .unwrap();
            str::from_utf8(&kernel_command.stdout)
                .unwrap()
                .to_string()
                .replace("\n", "")
        }
    }
}

fn append_uptime_field<'a>(value: i64, input: &'a mut String, suffix: &str) -> String {
    if value.gt(&0) {
        if !input.is_empty() {
            input.push_str(", ");
        }
        input.push_str(
            &format!(
                "{} {}",
                &value,
                suffix,
            )
        );
    }
    input.clone()
}
