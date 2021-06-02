extern crate colored;
extern crate chrono;
extern crate serde;

mod sys;

use sys::SysInfo;

fn main() {
    let sys_info = SysInfo::new();

    println!("{}", &sys_info);
}

// Create structs for Hardware and OS/Software related things
// impl Display for both and then use as fields in SysInfo struct


// get linux hostname:
// cat /etc/hostname
// get windows hostname:
// hostname

// get linux meminfo:
// grep 'Mem[^A]' /proc/meminfo
