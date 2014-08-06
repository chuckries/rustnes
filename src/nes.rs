#![crate_name = "nes"]
#![crate_type = "lib"]

use types::{byte, word};

mod cpu;
mod mem;
mod types;

pub struct NES {
    num: word
}

impl NES {
    pub fn new() -> NES {
        NES{ num: 0xBEEF }
    }

    pub fn print(&self) {
        println!("0x{:X}", self.num);
    }
}
