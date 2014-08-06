use types::{byte};

pub struct Cart {
    name: String,

    rom_data: Vec<[byte, ..0x100]>
}

impl Cart {
  pub fn new() -> Cart {
      let mut data: Vec<[byte, ..0x100]> = Vec::new();
      data.push([0, ..0x100]);

      Cart{ 
          name: String::from_str("Hello"),
          rom_data: data
      }
  }
}
