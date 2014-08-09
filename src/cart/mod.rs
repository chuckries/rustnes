use std::io::File;
use std::mem;

#[cfg(test)]
mod test;

static PRG_ROM_BANK_SIZE: uint = 16 * 1024; //16 KB
static CHR_ROM_BANK_SIZE: uint = 8 * 1024; //8 KB

#[packed]
struct CartHeader {
    //TODO These don't all need to be public
    //currenlty it is just for testing, which will change
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

impl CartHeader {
    pub fn new(bytes: &[u8, ..0x10]) -> Option<CartHeader> {
        let cart_header: &CartHeader;

        unsafe {
            cart_header = mem::transmute(bytes.as_ptr());
        }

        if cart_header.is_valid() {
            Some(*cart_header)
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        //TODO Implement this mofo, too lazy right now
        true
    }
}

pub struct Cart {
    header: CartHeader,
    rom_data: Vec<[u8, ..0x100]>,
}

impl Cart {
    pub fn new(rom: &Path) -> Cart {
        let mut file = File::open(rom).unwrap();

        //get the header info
        let mut buf = [0u8, ..0x10];
        let bytes_read = file.read(buf).unwrap();
        let header = CartHeader::new(&buf).unwrap();
        
        //TODO read rest of the binary
        let mut data: Vec<[u8, ..0x100]> = Vec::new();

        Cart{ 
            header: header,
            rom_data: data,
        }
    }
}
