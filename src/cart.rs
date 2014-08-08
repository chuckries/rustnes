use std::io::{File, Open, Read};

use std::mem;

pub struct Cart {
    rom_data: Vec<[u8, ..0x100]>
}

#[packed]
pub struct CartHeader {
    pub identifier: [u8, ..4], // NES^
    pub prg_rom_count: u8, // in 16KB units
    pub chr_rom_count: u8, // in 8KB units
    pub flags_6: u8,
    pub flags_7: u8,
    pub prg_ram_count: u8, // in 8KB, minimum 8KB for compat
    pub flags_9: u8,
    pub flags_10: u8,
    pub zeros: [u8, ..5],
}

impl Cart {
    pub fn new(rom: &Path) -> Cart {
        let mut file = File::open(rom).unwrap();

        let mut buf = [0u8, ..0x10];

        let bytes_read = file.read(buf.as_mut_slice()).unwrap();
        
        let mut data: Vec<[u8, ..0x100]> = Vec::new();

        Cart{ 
            rom_data: data
        }
    }

    pub fn decode_header(header_bytes: &[u8]) -> CartHeader {
        let cart_header: &CartHeader;
        assert_eq!(header_bytes.len(), 0x10);
        unsafe {
            cart_header = mem::transmute(header_bytes.as_ptr());
        }
        *cart_header
    }
}

impl CartHeader {
    pub fn prg_rom_size(&self) -> uint {
        self.prg_rom_count as uint * 16u
    }

    pub fn chr_rom_size(&self) -> uint {
        self.chr_rom_count as uint * 8u
    }

    pub fn prg_ram_size(&self) -> uint {
        if self.prg_ram_count as uint == 0u {
            8u
        } else {
            self.prg_ram_count as uint * 8u
        }
    }
}
