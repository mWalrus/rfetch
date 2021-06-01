extern crate colored;
extern crate time;
extern crate chrono;

mod sys;

use sys::SysInfo;

fn main() {
    let sys_info = SysInfo::new();

    println!("{}", &sys_info);
}
