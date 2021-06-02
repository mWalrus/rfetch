use crate::chrono::offset::{TimeZone, Utc};
use std::{process::Command, str};
use chrono::Duration;
use regex::Regex;

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

pub fn uptime() -> Option<String> {
    let value_regex = Regex::new(r"\s\s(\w+\s?)+").unwrap();
    let utc = Utc;
    let now = Utc::now();
    match cfg!(windows) {
        true => {
            let boot_date_time_cmd = Command::new("cmd")
                .args(vec![
                    "/C",
                    "wmic path Win32_OperatingSystem get LastBootUpTime"
                ])
                .output()
                .unwrap();
            let boot_date_time_raw = str::from_utf8(&boot_date_time_cmd.stdout)
                .unwrap()
                .to_owned();
            let mut boot_date_time_fmt = value_regex.captures(&boot_date_time_raw)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .replace("  ", "");
            let millis_regex = Regex::new(r"(\.|,)\d{6}(\+|-)\d{2,3}").unwrap();
            boot_date_time_fmt = millis_regex
                .replace(&boot_date_time_fmt, "")
                .to_string();
            let boot = utc.datetime_from_str(&boot_date_time_fmt, "%Y%m%d%H%M%S").unwrap();
            let up_since_boot = now.signed_duration_since(boot);
            // break out values from above
            format(&up_since_boot)
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
            Some(uptime)
        }
    }
}

pub fn format(duration: &Duration) -> Option<String> {
    let input_seconds = duration.num_seconds();
    let seconds_in_minute = 60;
    let seconds_in_hour = seconds_in_minute * 60;
    let seconds_in_day = seconds_in_hour * 24;
    let seconds_in_month = seconds_in_day * 31;
    let seconds_in_year = seconds_in_month * 12;

    let years = input_seconds/seconds_in_year;

    let mut remaining_seconds = input_seconds - (years * seconds_in_year);
    let months = remaining_seconds/seconds_in_month;

    remaining_seconds = remaining_seconds - (months * seconds_in_month);
    let days = remaining_seconds/seconds_in_day;

    remaining_seconds = remaining_seconds - (days * seconds_in_day);
    let hours = remaining_seconds/seconds_in_hour;

    remaining_seconds = remaining_seconds - (hours * seconds_in_hour);
    let minutes = remaining_seconds/seconds_in_minute;

    remaining_seconds = remaining_seconds - (minutes * seconds_in_minute);
    let seconds = remaining_seconds;

    let mut output = String::new();
    output = append_field(years, &mut output, "year");
    output = append_field(months, &mut output, "month");
    output = append_field(days, &mut output, "day");
    output = append_field(hours, &mut output, "hour");
    output = append_field(minutes, &mut output, "minute");
    output = append_field(seconds, &mut output, "second");
    if output.is_empty() { None } else { Some(output) }
}

fn append_field<'a>(value: i64, input: &'a mut String, suffix: &str) -> String {
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
