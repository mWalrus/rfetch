use std::{fmt, fs::File, io::Read};

pub struct CPU {
    pub vendor: String,
    pub brand: String,
    pub mhz: f32,
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu_info_string = String::new();
        match File::open("/proc/cpuinfo") {
            Ok(info) => info.read_to_string(&mut cpu_info_string),
            Err(e) => panic!("Failed to read cpu info: {}", e),
        };
        let line_split = cpu_info_string.split("\n").collect::<Vec<_>>();

    }

    pub fn mhz_to_ghz(&self) -> u16 {

    }
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} @ {}", self.vendor, self.brand, self.mhz)
    }
}
