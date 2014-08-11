//! # NES Memory module
//!
//! ## Memory Map
//! This is from http://nesdev.com/NESDoc.pdf
//!  _______________         _______________
//! | PRG-ROM       |       |               |
//! | Upper Bank    |       |               |
//! |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
//! | PRG-ROM       |       |               |
//! | Lower Bank    |       |               |
//! |_______________| $8000 |_______________|
//! | SRAM          |       | SRAM          |
//! |_______________| $6000 |_______________|
//! | Expansion ROM |       | Expansion ROM |
//! |_______________| $4020 |_______________|
//! | I/O Registers |       |               |
//! |_ _ _ _ _ _ _ _| $4000 |               |
//! | Mirrors       |       | I/O Registers |
//! | $2000-$2007   |       |               |
//! |_ _ _ _ _ _ _ _| $2008 |               |
//! | I/O Registers |       |               |
//! |_______________| $2000 |_______________|
//! | Mirrors       |       |               |
//! | $0000-$07FF   |       |               |
//! |_ _ _ _ _ _ _ _| $0800 |               |
//! | RAM           |       | RAM           |
//! |_ _ _ _ _ _ _ _| $0200 |               |
//! | Stack         |       |               |
//! |_ _ _ _ _ _ _ _| $0100 |               |
//! | Zero Page     |       |               |
//! |_______________| $0000 |_______________|

use cart::Cart;

static ZERO_PAGE_SIZE:  uint = 0x100; //256 bytes
static STACK_SIZE:      uint = 0x100; //256 bytes
static RAM_SIZE:        uint = 0x600; //1.5 KB

pub struct Mem {
    //used for reading PRG_ROM (and others?) from the cartridge
    cart: Cart, 

    zero_page:  [u8, ..ZERO_PAGE_SIZE],
    stack:      [u8, ..STACK_SIZE],
    ram:        [u8, ..RAM_SIZE],

}

impl Mem {
    pub fn new(cart: Cart) -> Mem {
        Mem {
            cart: cart,

            zero_page:  [0u8, ..ZERO_PAGE_SIZE],
            stack:      [0u8, ..STACK_SIZE],
            ram:        [0u8, ..RAM_SIZE],
        }
    }

    //TODO lots
    pub fn read(&self, virtual_address: u16) -> u8 {
        if virtual_address < 0x2000 {
            let address: uint = (virtual_address as uint) & 0x07FFF; //Mirrored after 0x0800

            if address < 0x0100 {
                self.zero_page[address]
            } else if address < 0x0200 {
                self.stack[address & 0xFF]
            } else if address < 0x0800 {
                self.ram[address & 0x01FF]
            } else {
                error!("Impossible");
                0x00
            }
        } else if virtual_address < 0x4000 {
            let address: uint = (virtual_address as uint) & 0x0007; //Mirrored after 0x2008
            //TODO calls into PPU at this point
            //TODO several of these registers are read only
            match address {
                0 => { 0 } //PPU Control Register 1
                1 => { 0 } //PPU Control Register 2
                2 => { 0 } //PPU Status Register
                3 => { 0 } //SPR-RAM Address Register
                4 => { 0 } //SPR-RAM I/O Register
                5 => { 0 } //VRAM Address Register 1
                6 => { 0 } //VRAM Address Register 2
                7 => { 0 } //VRAM I/O Register
                _ => { error!("Impossible"); 0x00 }
            }
        } else if virtual_address < 0x4020 {
            //TODO APU Registers 
            0x00
        } else if virtual_address < 0x6000 {
            //TODO Expansion ROM
            0x00
        } else if virtual_address < 0x8000 {
            //TODO SRAM
            0x00
        } else if virtual_address < 0xC000 {
            self.cart.read_from_upper_bank(virtual_address & 0x3FFF)
        } else if virtual_address <= 0xFFFF {
            self.cart.read_from_lower_bank(virtual_address & 0x3FFF)
        } else {
            error!("Impossible");
            0x00
        }
    }
}

