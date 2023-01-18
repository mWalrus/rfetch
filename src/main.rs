use chrono::Duration;
use colored::{ColoredString, Colorize};
use std::{
    fmt,
    io::{self, stdout, Write},
};
use sysinfo::{
    get_current_pid, Pid, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt, User,
    UserExt,
};

struct Uptime(u64);

impl fmt::Display for Uptime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = Duration::seconds(self.0 as i64);

        write!(
            f,
            "{:0>2}h {:0>2}m {:0>2}s",
            d.num_hours(),
            d.num_minutes() % 60,
            d.num_seconds() % 60,
        )
    }
}

#[rustfmt::skip]
fn main() -> io::Result<()> {
    // only read specifics
    let mut sys = System::new_with_specifics(RefreshKind::new().with_memory().with_users_list());

    let pid = get_current_pid().unwrap();

    // only fetch the information for the current process id
    sys.refresh_process_specifics(pid, ProcessRefreshKind::new().with_user());

    let user = evaluate_invoking_user(&sys, pid);

    // take lock on stdout for entire printing session (faster)
    let mut lock = stdout().lock();

    write!(lock, "{}@{}\n",           user.name().bright_magenta().bold(), value(sys.host_name()).magenta().bold())?;
    write!(lock, "{}\t{}\n",          title("os"), value(sys.name()))?;
    write!(lock, "{}\t{}\n",          title("kernel"), value(sys.kernel_version()))?;
    write!(lock, "{}\t{}\n",          title("uptime"), Uptime(sys.uptime()))?;
    write!(lock, "{}\t{}MiB/{}MiB\n", title("mem"), sys.used_memory() / 1024 / 1024, sys.total_memory() / 1024 / 1024)?;

    Ok(())
}

fn title(t: &'static str) -> ColoredString {
    t.bold().bright_blue()
}

fn value<T: Default>(v: Option<T>) -> T {
    v.unwrap_or_default()
}

fn evaluate_invoking_user(s: &System, pid: Pid) -> &User {
    let proc = s.process(pid).expect("Failed to get process");
    let uid = proc
        .user_id()
        .expect("Failed to get the user owning the current process");

    s.get_user_by_id(uid)
        .expect("Failed to get user with id {uid:?}")
}
