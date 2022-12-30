use chrono::Duration;
use colored::{ColoredString, Colorize};
use std::io::{self, stdout, Write};
use sysinfo::{
    get_current_pid, Pid, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt, User,
    UserExt,
};

fn main() -> io::Result<()> {
    // only read specifics
    let mut sys = System::new_with_specifics(RefreshKind::new().with_memory().with_users_list());

    let pid = get_current_pid().unwrap();

    // only fetch the information for the current process id
    sys.refresh_process_specifics(pid, ProcessRefreshKind::new().with_user());

    let user = match evaluate_invoking_user(&sys, pid) {
        Ok(u) => u,
        Err(e) => panic!("{}", e),
    };

    // take lock on stdout for entire printing session (faster)
    let mut lock = stdout().lock();

    write!(
        lock,
        "{}@{}\n",
        user.name().bright_magenta().bold(),
        sys.host_name().unwrap_or_default().magenta().bold(),
    )?;

    write!(lock, "{}\t{}\n", title("os"), value(sys.name()))?;
    write!(
        lock,
        "{}\t{}\n",
        title("kernel"),
        value(sys.kernel_version())
    )?;
    write!(lock, "{}\t{}\n", title("uptime"), uptime_fmt(sys.uptime()))?;
    write!(
        lock,
        "{}\t{}MiB/{}MiB\n",
        title("mem"),
        sys.used_memory() / 1024 / 1024,
        sys.total_memory() / 1024 / 1024
    )?;

    Ok(())
}

fn title(t: &'static str) -> ColoredString {
    t.bold().bright_blue()
}

fn value<T: Default>(v: Option<T>) -> T {
    v.unwrap_or_default()
}

fn evaluate_invoking_user(s: &System, pid: Pid) -> Result<&User, String> {
    let proc = match s.process(pid) {
        Some(p) => p,
        None => Err(format!("Failed to get process with pid {pid}"))?,
    };

    let uid = match proc.user_id() {
        Some(id) => id,
        None => Err(format!(
            "Failed to get the user owning process with pid {pid}"
        ))?,
    };
    match s.get_user_by_id(uid) {
        Some(user) => Ok(user),
        None => Err(format!("Failed to get user with id {uid:?}"))?,
    }
}

fn uptime_fmt(u: u64) -> String {
    let mut dur = Duration::seconds(u as i64);

    let hours = dur.num_hours();
    dur = dur - Duration::hours(hours);

    let minutes = dur.num_minutes();
    dur = dur - Duration::minutes(minutes);

    let seconds = dur.num_seconds();

    let h = if hours == 0 {
        "".into()
    } else {
        format!("{}h", hours)
    };
    let m = if hours == 0 {
        "".into()
    } else {
        format!("{}m", minutes)
    };
    let s = if hours == 0 {
        "".into()
    } else {
        format!("{}s", seconds)
    };

    format!("{h} {m} {s}")
}
