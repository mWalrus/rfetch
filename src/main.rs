extern crate colored;

mod cpu;

use cpu::CPU;
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
    let cpuinfo = CPU::get_cpu_info();
    print_line("CPU", cpuinfo);
}

fn print_line<T: std::fmt::Display>(title: &str, info: T) {
    println!("{} {}", format!("{}:", title).bold().blue(), &info);
}
