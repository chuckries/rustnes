#![crate_name = "nes"]
#![crate_type = "lib"]

mod cpu;
mod mem;
mod types;

pub type word = u16;
pub type byte = u8;

pub struct NES {
    num: int
}

impl NES {
    pub fn new() -> NES {
        NES{ num: 0xDEADBEEF }
    }

    pub fn print(&self) {
        println!("0x{:X}", self.num);
    }
}
