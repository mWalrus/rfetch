extern crate colored;
extern crate chrono;
extern crate serde;

use std::{process::Command, str};
use colored::Colorize;
use regex::Regex;

fn mem_info() -> String {
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
    str::from_utf8(&mem_command.stdout)
        .unwrap()
        .replace('\n', "")
}

fn header() -> (String, String) {
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
    (user, hostname)
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
    }
}

pub fn kernel() -> String {
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
    str::from_utf8(&kernel_command.stdout)
        .unwrap()
        .replace('\n', "")
}

fn main() {
    let (user, hostname) = header();
    println!(
        "{}@{}\n{}\t{}\n{}\t{}\n{}\t{}\n{}\t{}",
        user.bold().magenta(), hostname.bold().magenta(),
        "mem".bold().blue(), os_name().truecolor(180, 180, 180),
        "kernel".bold().blue(), kernel().truecolor(180, 180, 180),
        "uptime".bold().blue(), uptime().truecolor(180, 180, 180),
        "mem".bold().blue(), mem_info().truecolor(180, 180, 180)
    )
}
