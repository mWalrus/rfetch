extern crate colored;
extern crate chrono;
extern crate serde;

mod sys;

use sys::SysInfo;

fn main() {
    let sys_info = SysInfo::new();

    println!("{}", &sys_info);
}
