use std::{process::Command, str};
use regex::Regex;
use std::collections::HashMap;

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
            let name = stdout.replace("Caption=", "").replace("\n", "");
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
                let mut iter = replace_regex.splitn(&field, 1);
                let key = iter.next()
                    .unwrap()
                    .to_lowercase()
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
        if value.gt(&1) {
            input.push('s');
        }

    }
    input.clone()
}
