use std::io::{File, Open, Read};

use types::{byte};

pub struct Cart {
    rom_data: Vec<[byte, ..0x100]>
}

impl Cart {
  pub fn new(rom: &Path) -> Cart {
      let mut file = File::open(rom).unwrap();

      let mut buf = [0u8, ..0x10];

      let bytes_read = file.read(buf.as_mut_slice()).unwrap();

      let mut data: Vec<[byte, ..0x100]> = Vec::new();

      Cart{ 
          rom_data: data
      }
  }
}
