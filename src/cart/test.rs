use super::CartHeader;

//TODO I'm almost certain this is defined in libstd somewhere
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

fn get_empty_header() -> CartHeader {
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

#[test]
fn header_decode_test() {
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
fn header_is_valid_test() {
    let hdr = CartHeader::new(&TEST_ROM_HEADER).unwrap();
    assert!(hdr.is_valid());

    let bad_hdr_bytes = [0u8, ..0x10];
    let bad_hdr = CartHeader::new(&bad_hdr_bytes);
    assert!(bad_hdr.is_none());

    let bad_hdr = get_empty_header();
    assert_eq!(bad_hdr.is_valid(), false);
}
