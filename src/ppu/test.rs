use std::iter;
use std::mem;

use ppu::{Spr, SprRam};

#[test]
fn ppu_spr_test() {
    let spr_bytes = [0u8, ..4];
    let spr = Spr::new(spr_bytes);
    assert_eq!(spr.X, 0);
    assert_eq!(spr.Y, 0);
    assert_eq!(spr.I, 0);
    assert_eq!(spr.color(), 0x00);
    assert!(spr.has_priority() == false);
    assert!(spr.h_flip() == false);
    assert!(spr.v_flip() == false);

    let spr_bytes = [0xAA, 0xBB, 0xCC, 0xDD];
    let spr = Spr::new(spr_bytes);
    assert_eq!(spr.X, 0xDD);
    assert_eq!(spr.Y, 0xAA);
    assert_eq!(spr.I, 0xBB);
    assert_eq!(spr.attr.bits, 0xCC);

    let spr_bytes = [0u8, ..4];
    let mut spr = Spr::new(spr_bytes);
    spr.attr.bits = 0b00000001;
    assert_eq!(spr.color(), 0b00000100);
    spr.attr.bits = 0b00000010;
    assert_eq!(spr.color(), 0b00001000);
    spr.attr.bits = 0b00000011;
    assert_eq!(spr.color(), 0b00001100);

    spr.attr.bits = 0b00100000;
    assert!(spr.has_priority());
    spr.attr.bits = 0b01000000;
    assert!(spr.h_flip());
    spr.attr.bits = 0b01100000;
    assert!(spr.h_flip() && spr.has_priority());
    spr.attr.bits = 0b10000000;
    assert!(spr.v_flip());
    spr.attr.bits = 0b10100000;
    assert!(spr.v_flip() && spr.has_priority());
    spr.attr.bits = 0b11000000;
    assert!(spr.v_flip() && spr.h_flip());
    spr.attr.bits = 0b11100000;
    assert!(spr.v_flip() && spr.h_flip() && spr.has_priority());
}

#[test]
fn ppu_spr_ram_test() {
    let spr_ram_bytes: [u8, ..256] = unsafe { mem::transmute([[0xAAu8, 0xBB, 0xCC, 0xDD], ..64]) };

    for i in iter::range_step(0, spr_ram_bytes.len(), 4) {
        assert_eq!(spr_ram_bytes[i], 0xAA);
        assert_eq!(spr_ram_bytes[i + 1], 0xBB);
        assert_eq!(spr_ram_bytes[i + 2], 0xCC);
        assert_eq!(spr_ram_bytes[i + 3], 0xDD);
    }

    let SprRam(spr_ram) = SprRam::new(spr_ram_bytes);

    for i in range(0, 64) { 
        assert_eq!(spr_ram[i].Y, 0xAA);
        assert_eq!(spr_ram[i].I, 0xBB);
        assert_eq!(spr_ram[i].attr.bits, 0xCC);
        assert_eq!(spr_ram[i].X, 0xDD);
    }
}
