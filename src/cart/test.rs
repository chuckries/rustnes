#![macro_escape]

use cart::{Cart, CartHeader};
use cart::{PrgRomBank, PrgRom};
use cart::{PRG_ROM_BANK_SIZE, CHR_ROM_BANK_SIZE};
use cart::{PRG_RAM_BANK_SIZE, TRAINER_SIZE};

static MSDOS_EOF: u8 = 0x1a;

static TEST_ROM_HEADER: [u8, ..16] = [ 
    0x4e, 0x45, 0x53, 0x1a, //NES^
    0x04, //PRG-ROM Count
    0x02, //CHR-ROM Count
    0x20, //Flags 6
    0x40, //Flags 7
    0x00, //PRG-RAM Count
    0x00, //Flags 9
    0x00, //Flags 10
    0x00, 0x00, 0x00, 0x00, 0x00,
];

macro_rules! cart_header(
    () => ( //Used as 'cart_header!()' to get an empty header
        get_empty_cart_header()
    );
)

macro_rules! cart(
    () => ( //Used as 'cart!()' to get an empty cart
        get_empty_cart()
    );
    ($prg_rom:expr) => ( //Used as 'cart!(prg_rom)' to get a cart initialized with prg_rom
        get_cart_with_prg_rom($prg_rom)
    );
)

macro_rules! prg_rom(
    () => ( //used as 'prg_rom!()' to get a vec of two empty prg_rom banks
        get_empty_prg_rom()
    );
    ($($e:expr),*) => ({
        let mut _temp = Vec::new();
        $(_temp.push($e);)*
        _temp
    });
    ($($e:expr),+,) => (prg_rom!($($e),+))
)

macro_rules! prg_rom_bank(
    () => ( //used as 'prg_rom_bank!()' to get an empty static array of size PRG_ROM_BANK_SIZE
        get_empty_prg_rom_bank()
    );
    ($init:expr) => ( //used as 'prg_rom_bank!(0xAA)' to get a static array of size PRG_ROM_BANK_SIZE with every entry initalized to 0xAA
        get_initialized_prg_rom_bank($init)
    );
)

pub fn get_empty_cart_header() -> CartHeader {
    CartHeader {
        identifier:     [0u8, ..4],
        prg_rom_count:  0u8,
        chr_rom_count:  0u8,
        flags_6:        0u8,
        flags_7:        0u8,
        prg_ram_count:  0u8,
        flags_9:        0u8,
        flags_10:       0u8,
        zeros:          [0u8, ..5],
    }
}

pub fn get_empty_cart() -> Cart {
    let hdr = cart_header!();

    Cart {
        header: hdr,
        prg_rom: prg_rom!(),
        chr_rom: Vec::from_fn(1, |_| [0u8, ..CHR_ROM_BANK_SIZE]),
        _trainer: [0u8, ..TRAINER_SIZE],
    }
}

pub fn get_empty_prg_rom_bank() -> PrgRomBank {
    [0u8, ..PRG_ROM_BANK_SIZE]
}

pub fn get_initialized_prg_rom_bank(init: u8) -> PrgRomBank {
    [init, ..PRG_ROM_BANK_SIZE]
}

pub fn get_empty_prg_rom() -> PrgRom {
    Vec::from_fn(2, |_| prg_rom_bank!())
}

pub fn get_cart_with_prg_rom(prg_rom: PrgRom) -> Cart {
    let mut cart = cart!();
    cart.prg_rom = prg_rom;
    cart
}

#[test]
fn cart_header_decode_test() {
    let hdr = CartHeader::new(&TEST_ROM_HEADER).unwrap();
    assert_eq!(hdr.identifier[0], 'N' as u8);
    assert_eq!(hdr.identifier[1], 'E' as u8);
    assert_eq!(hdr.identifier[2], 'S' as u8);
    assert_eq!(hdr.identifier[3], MSDOS_EOF);
    assert_eq!(hdr.flags_6, 32);
    assert_eq!(hdr.flags_7, 64);
    assert_eq!(hdr.flags_9, 0);
    assert_eq!(hdr.flags_10, 0);
    assert_eq!(hdr.zeros[0], 0);
    assert_eq!(hdr.zeros[1], 0);
    assert_eq!(hdr.zeros[2], 0);
    assert_eq!(hdr.zeros[3], 0);
    assert_eq!(hdr.zeros[4], 0);
}

#[test]
fn cart_header_is_valid_test() {
    let hdr = CartHeader::new(&TEST_ROM_HEADER).unwrap();
    assert!(hdr.is_valid());

    let bad_hdr_bytes = [0u8, ..0x10];
    let bad_hdr = CartHeader::new(&bad_hdr_bytes);
    assert!(bad_hdr.is_none());

    let bad_hdr = cart_header!();
    assert_eq!(bad_hdr.is_valid(), false);
}

#[test]
fn cart_read_default_prg_rom_banks_test() {
    let prg_rom = vec![prg_rom_bank!(0xAA), prg_rom_bank!(0xBB)];

    let cart = cart!(prg_rom);
    for i in range(0, cart.prg_rom.len() as u16) {
        assert_eq!(cart.read_from_lower_bank(i), 0xAA as u8);
        assert_eq!(cart.read_from_upper_bank(i), 0xBB as u8);
    }
}
