//! # NES Memory module
//!
//! ## Memory Map
//! This is from http://nesdev.com/NESDoc.pdf
//!  _______________
//! | PRG-ROM       |
//! | Upper Bank    |
//! |_ _ _ _ _ _ _ _| $C000
//! | PRG-ROM       |
//! | Lower Bank    |
//! |_______________| $8000
//! | SRAM          |
//! |_______________| $6000
//! | Expansion ROM |
//! |_______________| $4020
//! | I/O Registers |
//! |_ _ _ _ _ _ _ _| $4000
//! | Mirrors       |
//! | $2000-$2007   |
//! |_ _ _ _ _ _ _ _| $2008
//! | I/O Registers |
//! |_______________| $2000
//! | Mirrors       |
//! | $0000-$07FF   |
//! |_ _ _ _ _ _ _ _| $0800
//! | RAM           |
//! |_ _ _ _ _ _ _ _| $0200
//! | Stack         | 
//! |_ _ _ _ _ _ _ _| $0100
//! | Zero Page     |
//! |_______________| $0000

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

    pub fn read(&self, addr: u16) -> u8 {
        0x00
    }
}

