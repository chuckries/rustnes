use cart::{Cart, CartHeader};
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

pub fn get_empty_header() -> CartHeader {
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
    let hdr = get_empty_header();

    Cart {
        header: hdr,
        prg_rom: get_empty_prg_rom(),
        chr_rom: Vec::from_fn(1, |_| [0u8, ..CHR_ROM_BANK_SIZE]),
        _trainer: [0u8, ..TRAINER_SIZE],
    }
}

pub fn get_empty_prg_rom() -> Vec<[u8, ..PRG_ROM_BANK_SIZE]> {
    Vec::from_fn(2, |_| [0u8, ..PRG_ROM_BANK_SIZE])
}

pub fn get_cart_with_prg_rom(prg_rom: Vec<[u8, ..PRG_ROM_BANK_SIZE]>) -> Cart {
    let mut cart = get_empty_cart();
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

    let bad_hdr = get_empty_header();
    assert_eq!(bad_hdr.is_valid(), false);
}

#[test]
fn cart_read_default_prg_rom_banks_test() {
    let mut cart = get_empty_cart();

    let prg_rom = vec![[0xAA, ..PRG_ROM_BANK_SIZE], [0xBB, ..PRG_ROM_BANK_SIZE]];
    cart.prg_rom = prg_rom;
    for i in range(0, PRG_ROM_BANK_SIZE as u16) {
        assert_eq!(cart.read_from_lower_bank(i), 0xAA as u8);
        assert_eq!(cart.read_from_upper_bank(i), 0xBB as u8);
    }
}
