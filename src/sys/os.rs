use crate::chrono::{NaiveDateTime, offset::{TimeZone, Utc}};
use crate::time;
use std::{process::Command, str};

use chrono::DateTime;
use regex::Regex;
use time::PrimitiveDateTime;

pub struct Uptime {
    years: Option<u8>,
    months: Option<u8>,
    days: Option<u8>,
    hours: Option<u8>,
    minutes: Option<u8>,
    seconds: Option<u8>,
}

impl Uptime {
    pub fn new(
        years: Option<u8>,
        months: Option<u8>,
        days: Option<u8>,
        hours: Option<u8>,
        minutes: Option<u8>,
        seconds: Option<u8>,
    ) -> Uptime {
        Uptime {
            years,
            months,
            days,
            hours,
            minutes,
            seconds,
        }
    }

    pub fn format(&self) -> String {
        let mut output = String::new();
        output = append_field(self.years, &mut output, "year");
        output = append_field(self.months, &mut output, "month");
        output = append_field(self.days, &mut output, "day");
        output = append_field(self.hours, &mut output, "hour");
        output = append_field(self.minutes, &mut output, "minute");
        output = append_field(self.seconds, &mut output, "second");
        output
    }
}

pub fn os_name() -> String {
    match cfg!(windows) {
        true => {
            let value_regex = Regex::new(r"\s\s(\w+\s?)+").unwrap();
            let name_cmd = Command::new("cmd")
                .args(vec![
                    "/C",
                    "systeminfo | findstr /B /C:'OS Name'"
                ])
                .output()
                .unwrap();
            let mut name = str::from_utf8(&name_cmd.stdout)
                .unwrap()
                .to_owned();
            name = value_regex.captures(&name)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .replace("  ", "");

            name.to_owned()
        },
        false => {
            String::new()
        }
    }
}

fn uptime(val_regex: Regex) -> Uptime {
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
            let mut boot_date_time_fmt = val_regex.captures(&boot_date_time_raw)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .replace("  ", "");
            let millis_regex = Regex::new(r"(\.|,)\d{6}(\+|-)\d{2,3}").unwrap();
            boot_date_time_fmt = millis_regex.replace(&boot_date_time_fmt, "").to_string();
            let boot = utc.datetime_from_str(&boot_date_time_fmt, "%Y%m%d%H%M%S").unwrap();
            let up_since_boot = now.signed_duration_since(boot);
            // break out values from above
        },
        false => {

        }
    }

    Uptime::new(None, None, None, None, None, None)
}

fn append_field<'a>(value: Option<u8>, input: &'a mut String, suffix: &str) -> String {
    if value.is_some() {
        let unwrapped = value.unwrap();
        if !input.is_empty() {
            input.push_str(", ");
        }
        input.push_str(
            &format!(
                "{} {}",
                &unwrapped,
                suffix,
            )
        );
        if unwrapped > 1 {
            input.push('s');
        }

    }
    input.clone()
}
