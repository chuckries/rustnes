#![macro_escape]

use nes::{
    PRG_ROM_BANK_SIZE,
    PrgRomBank,
    PrgRom,
    RomHeader,
};

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

macro_rules! rom_header(
    () => ( 
        get_empty_rom_header()
    );
)

macro_rules! prg_rom(
    () => ( //used as 'prg_rom!()' to get a vec of two empty prg_rom banks
        Vec::from_fn(2, |_| prg_rom_bank!())
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
        //get_empty_prg_rom_bank()
        [0u8, ..PRG_ROM_BANK_SIZE]
    );
    ($init:expr) => ( //used as 'prg_rom_bank!(0xAA)' to get a static array of size PRG_ROM_BANK_SIZE with every entry initalized to 0xAA
        //get_initialized_prg_rom_bank($init)
        [$init, ..PRG_ROM_BANK_SIZE]
    );
)

pub fn get_empty_rom_header() -> RomHeader {
    RomHeader {
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

#[test]
fn nes_rom_header_decode_test() {
    let hdr = RomHeader::new(&TEST_ROM_HEADER).unwrap();
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
fn nes_rom_header_is_valid_test() {
    let hdr = RomHeader::new(&TEST_ROM_HEADER).unwrap();
    assert!(hdr.is_valid());

    let bad_hdr_bytes = [0u8, ..0x10];
    let bad_hdr = RomHeader::new(&bad_hdr_bytes);
    assert!(bad_hdr.is_none());

    let bad_hdr = rom_header!();
    assert_eq!(bad_hdr.is_valid(), false);
}
