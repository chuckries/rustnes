use std::iter;
use std::mem;

use nes::VAddr;

use ppu::{
    Spr,
    SprRam,
    SPR_RAM_SIZE,
    SPR_PRIORITY_FLAG,
    SPR_H_FLIP,
    SPR_V_FLIP,
};

#[test]
fn ppu_spr_test() {
}

#[test]
fn ppu_spr_ram_test() {
    let mut spr_ram = SprRam::new();

    for i in range(0, spr_ram.buf.len() as VAddr) {
        assert_eq!(spr_ram[i], 0x00);
    }

    spr_ram.buf[0x00] = 0xAA;
    spr_ram.buf[0x01] = 0xBB;
    spr_ram.buf[0x02] = SPR_PRIORITY_FLAG | SPR_H_FLIP | SPR_V_FLIP | 0b00000011;
    spr_ram.buf[0x03] = 0xDD;

    assert_eq!(spr_ram[0x00], 0xAA);
    assert_eq!(spr_ram[0x01], 0xBB);
    assert_eq!(spr_ram[0x02], 0b11100011);
    assert_eq!(spr_ram[0x03], 0xDD);

    let spr = spr_ram.spr(0);

    assert_eq!(spr.y(), 0xAA);
    assert_eq!(spr.x(), 0xDD);
    assert_eq!(spr.idx(), 0xBB);
    assert_eq!(spr.color(), 0b00001100);
    assert_eq!(spr.has_priority(), true);
    assert_eq!(spr.h_flip(), true);
    assert_eq!(spr.v_flip(), true);
}
