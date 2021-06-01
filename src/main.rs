extern crate colored;

mod sys;

use sys::hw::{CPU, GPU};
use sys::os::OS;
use colored::*;

trait Dedup<T: PartialEq + Clone> {
    fn clear_duplicates(&mut self);
}

impl<T: PartialEq + Clone> Dedup<T> for Vec<T> {
    fn clear_duplicates(&mut self) {
        let mut already_seen = Vec::with_capacity(self.len());
        self.retain(|item| match already_seen.contains(item) {
            true => false,
            _ => {
                already_seen.push(item.clone());
                true
            }
        })
    }
}

fn main() {
    let cpuinfo = CPU::get_info();
    let gpuinfo = GPU::get_info();

    print_line("CPU", cpuinfo);
    print_line("GPU", gpuinfo);
}

fn print_line<T: std::fmt::Display>(title: &str, info: T) {
    println!("{} {}", format!("{}:", title).bold().blue(), &info);
}
