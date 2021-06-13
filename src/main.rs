extern crate colored;
extern crate chrono;
extern crate serde;

mod hw;
mod os;

use hw::HW;
use os::OS;

fn main() {
    let hardware = HW::new();
    let os = OS::new();
    println!("{}", &hardware);
    println!("{}", &os);
}

// Create structs for Hardware and OS/Software related things
// impl Display for both and then use as fields in SysInfo struct


// get linux hostname:
// cat /etc/hostname
// get windows hostname:
// hostname

// get linux meminfo:
// grep 'Mem[^A]' /proc/meminfo
