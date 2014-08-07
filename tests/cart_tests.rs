extern crate rustnes;

use rustnes::cart::{Cart};

static MSDOS_EOF: u8 = 0x1a;

static TEST_ROM_HEADER: [u8, ..16] = [ 0x4eu8, 0x45, 0x53, 0x1a, 0x04, 0x02, 0x20, 0x40,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 ];

#[test]
fn header_decode_test() {
    let hdr = Cart::decode_header(TEST_ROM_HEADER.as_slice());
    assert_eq!(hdr.identifier[0], 'N' as u8);
    assert_eq!(hdr.identifier[1], 'E' as u8);
    assert_eq!(hdr.identifier[2], 'S' as u8);
    assert_eq!(hdr.identifier[3], MSDOS_EOF);
    assert_eq!(hdr.prg_rom_size(), 64);
    assert_eq!(hdr.chr_rom_size(), 16);
    assert_eq!(hdr.flags_6, 32);
    assert_eq!(hdr.flags_7, 64);
    assert_eq!(hdr.prg_ram_size(), 8);
    assert_eq!(hdr.flags_9, 0);
    assert_eq!(hdr.flags_10, 0);
    assert_eq!(hdr.zeros[0], 0);
    assert_eq!(hdr.zeros[1], 0);
    assert_eq!(hdr.zeros[2], 0);
    assert_eq!(hdr.zeros[3], 0);
    assert_eq!(hdr.zeros[4], 0);
}