use std::io::{File, Open, Read};

use types::{byte};

pub struct Cart {
    rom_data: Vec<[byte, ..0x100]>
}

impl Cart {
  pub fn new(rom: &Path) -> Cart {
      let mut file = match File::open(rom) {
          Ok(f) => f,
          Err(e) => fail!("file error: {}", e)
      };

      let mut buf = [0u8, ..0x10];

      let bytes_read = match file.read(buf.as_mut_slice()) {
          Ok(n) => n,
          Err(e) => fail!("file read error: {}", e)
      };

      let mut data: Vec<[byte, ..0x100]> = Vec::new();

      Cart{ 
          rom_data: data
      }
  }
}
