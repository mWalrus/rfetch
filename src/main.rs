extern crate colored;
extern crate chrono;
extern crate serde;

use std::{process::Command, str};
use colored::Colorize;
use regex::Regex;

#[cfg(target_os = "windows")]
fn mem_info() -> String {
    
    // We get the mem free and mem total and calculate from that
    let cmd = Command::new("powershell")
        .args(vec![
            "-Command",
            "Get-CIMInstance Win32_OperatingSystem | % {'{0}MiB / {1}MiB' -f [Int](($_.TotalVisibleMemorySize - $_.FreePhysicalMemory)*0.000953674), [Int]($_.TotalVisibleMemorySize*0.000953674)}"
        ])
        .output()
        .unwrap();
    
    str::from_utf8(&cmd.stdout)
        .unwrap()
        .replace('\n', "")
}

#[cfg(target_os = "linux")]
fn mem_info() -> String {
    let cmd = Command::new("bash")
        .args(vec![
            "-c",
            "free -m | awk -v OFS=' / ' -vsuf='MiB' '/Mem:/ {print $3 suf, $2 suf}'"
        ])
        .output()
        .unwrap();
    
    str::from_utf8(&cmd.stdout)
        .unwrap()
        .replace('\n', "")
}

#[cfg(target_os = "windows")]
fn header() -> (String, String) {
    let cmd = Command::new("cmd")
        .args(vec![
            "/C",
            "echo %username%@%computername%"
        ])
        .output()
        .unwrap();
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

#[cfg(target_os = "linux")]
fn header() -> (String, String) {
    let cmd = Command::new("bash")
        .args(vec![
            "-c",
            "echo \"`whoami`@`cat /proc/sys/kernel/hostname`\""
        ])
        .output()
        .unwrap();
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

#[cfg(target_os = "windows")]
pub fn os_name() -> String {
   let cmd = Command::new("cmd")
        .args(vec![
            "/C",
            "wmic os get Caption /value"
        ])
        .output()
        .unwrap();
    
    let stdout = str::from_utf8(&cmd.stdout)
        .unwrap()
        .to_owned();
    
    stdout.replace("Caption=", "")
        .replace('\n', "")
        .replace('\r', "")
}

pub fn os_name() -> String {
    let cmd = Command::new("bash")
        .args(vec![
            "-c",
            "grep '^NAME=' /etc/os-release | awk -F= '{print $2}' | sed 's/\"//g'"
        ])
        .output()
        .unwrap();
    
    str::from_utf8(&cmd.stdout)
        .unwrap()
        .replace('\n', "")
}

#[cfg(target_os = "windows")]
pub fn uptime() -> String {
    let replace_regex = Regex::new(r"\s{2,}:\s").unwrap();
    let cmd = Command::new("powershell")
        .args(vec![
            "-Command",
            "(Get-Date) - (Get-CimInstance -ClassName Win32_OperatingSystem).LastBootUpTime"
        ])
        .output()
        .unwrap();
    
    let raw = str::from_utf8(&cmd.stdout)
        .unwrap()
        .to_owned();
    
    // Output is formatted on multiple lines with one time variable for each line
    // so we need to split that output to handle each line.
    let cmd_output_split = raw.split('\n').collect::<Vec<_>>();
    
    let mut output = String::new();
    // Then we iterate through the items that we want to use.
    for field in cmd_output_split[2..=5].iter() {
        // We define an iterable item from the regex split
        let mut iter = replace_regex.splitn(&field, 2);
        
        // grab the key..
        let key = iter.next()
            .unwrap()
            .to_string();
        
        // and the value, which we parse into an i64 since its a number.
        let value = iter.next()
            .unwrap()
            .replace('\r', "")
            .parse::<i64>()
            .unwrap();
        
        // if the aforementioned value is greater than 0 it means that
        // we want to include the value in the output string.
        if value.gt(&0) {
            // formatting checks...
            if !output.is_empty() {
                output.push_str(", ");
            }
            // lastly we append to the output string.
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
}

#[cfg(target_os = "linux")]
pub fn uptime() -> String {
    let cmd = Command::new("bash")
        .args(vec![
            "-c",
            "uptime -p | sed 's/up //'"
        ])
        .output()
        .unwrap();
        
    str::from_utf8(&cmd.stdout)
        .unwrap()
        .replace('\n', "")
}

#[cfg(target_os = "windows")]
pub fn kernel() -> String {
    let cmd = Command::new("cmd")
        .args(vec![
            "/C",
            "echo %os%"
        ])
        .output()
        .unwrap();
    
    str::from_utf8(&cmd.stdout)
        .unwrap()
        .replace('\n', "")
}

#[cfg(target_os = "linux")]
pub fn kernel() -> String {
    let cmd = Command::new("bash")
        .args(vec![
            "-c",
            "uname -r"
        ])
        .output()
        .unwrap();

    str::from_utf8(&cmd.stdout)
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
