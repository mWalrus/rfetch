extern crate colored;
extern crate chrono;
extern crate serde;

mod hardware;
mod software;

use hardware::Hardware;
use software::{OS, UserHost};

fn main() {
    let user_hostname = UserHost::new();
    let os = OS::new();
    let hardware = Hardware::new();
    println!("{}", &user_hostname);
    println!("{}", &os);
    println!("{}", &hardware);
}

// Create structs for Hardware and OS/Software related things
// impl Display for both and then use as fields in SysInfo struct


// get linux hostname:
// cat /etc/hostname
// get windows hostname:
// hostname

// get linux meminfo:
// grep 'Mem[^A]' /proc/meminfo
