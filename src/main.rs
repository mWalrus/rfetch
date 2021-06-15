extern crate colored;
extern crate chrono;
extern crate serde;

use std::{process::Command, str};
use colored::Colorize;
use regex::Regex;

fn mem_info() {
    let mem_command = match cfg!(windows) {
        true => {
            // We get the mem free and mem total and calculate from that
            Command::new("powershell")
                .args(vec![
                    "-Command",
                    "Get-CIMInstance Win32_OperatingSystem | % {'{0}MiB / {1}MiB' -f [Int](($_.TotalVisibleMemorySize - $_.FreePhysicalMemory)*0.000953674), [Int]($_.TotalVisibleMemorySize*0.000953674)}"
                ])
                .output()
                .unwrap()
        },
        false => {
            Command::new("bash")
                .args(vec![
                    "-c",
                    "free -m | awk -v OFS=' / ' -vsuf='MiB' '/Mem:/ {print $3 suf, $2 suf}'"
                ])
                .output()
                .unwrap()
        }
    };
    let mem = str::from_utf8(&mem_command.stdout)
        .unwrap()
        .replace('\n', "");
    println!("{}\t{}", "mem".bold().blue(), mem.truecolor(180, 180, 180));
}

fn header() {
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
        .replace('\n', "")
        .replace('\r', "");
    println!("{}@{}", user.bold().magenta(), hostname.bold().magenta());
}

pub fn os_name() {
    let name = match cfg!(windows) {
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
            stdout.replace("Caption=", "")
                .replace('\n', "")
                .replace('\r', "")
        },
        false => {
            let name_cmd = Command::new("bash")
                .args(vec![
                    "-c",
                    "grep '^NAME=' /etc/os-release | awk -F= '{print $2}' | sed 's/\"//g'"
                ])
                .output()
                .unwrap();
            str::from_utf8(&name_cmd.stdout)
                .unwrap()
                .replace('\n', "")
        }
    };
    println!("{}\t{}", "os".bold().blue(), name.truecolor(180, 180, 180));
}

pub fn uptime() {
    let replace_regex = Regex::new(r"\s{2,}:\s").unwrap();
    let uptime = match cfg!(windows) {
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
            let uptime_split = uptime_raw.split('\n').collect::<Vec<_>>();
            let mut output = String::new();
            for field in uptime_split[2..=5].iter() {
                let mut iter = replace_regex.splitn(&field, 2);
                let key = iter.next()
                    .unwrap()
                    .to_string();
                let value = iter.next()
                    .unwrap()
                    .replace('\r', "")
                    .parse::<i64>()
                    .unwrap();
                if value.gt(&0) {
                    if !output.is_empty() {
                        output.push_str(", ");
                    }
                    output.push_str(
                        &format!(
                            "{} {}",
                            &value,
                            &key,
                        )
                    );
                }
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
            str::from_utf8(&uptime_cmd.stdout)
                .unwrap()
                .replace('\n', "")
        }
    };
    println!("{}\t{}", "uptime".bold().blue(), uptime.truecolor(180, 180, 180));
}

pub fn kernel() {
    let kernel_command = match cfg!(windows) {
        true => {
            Command::new("cmd")
                .args(vec![
                    "/C",
                    "echo %os%"
                ])
                .output()
                .unwrap()
        },
        false => {
            Command::new("bash")
                .args(vec![
                    "-c",
                    "uname -r"
                ])
                .output()
                .unwrap()
        }
    };
    let krnl = str::from_utf8(&kernel_command.stdout)
        .unwrap()
        .replace('\n', "");
    println!("{}\t{}", "kernel".bold().blue(), &krnl.truecolor(180, 180, 180));
}

fn main() {
    header();
    os_name();
    kernel();
    uptime();
    mem_info();
}
