use mem::{Mem, RAM_SIZE};
use cart::{Cart};
use cart::test::*;

pub fn get_empty_mem() -> Mem {
    let cart: Cart = get_empty_cart();

    Mem {
        cart: cart,
        ram: [0u8, ..RAM_SIZE],
    }
}

pub fn get_mem_with_cart(cart: Cart) -> Mem {
    Mem {
        cart: cart,
        ram: [0u8, ..RAM_SIZE],
    }
}

#[test]
fn mem_sanity_test() {
    let mut mem = get_empty_mem();

    mem.ram[0x0000] = 0xAA;
    mem.ram[0x0001] = 0xBB;

    assert_eq!(mem.read_byte(0x0000), 0xAA);
    assert_eq!(mem.read_byte(0x0001), 0xBB);

    assert_eq!(mem.read_word(0x0000), 0xBBAA);
}

#[test]
fn mem_ram_mirror_test() {
    let mut mem = get_empty_mem();

    mem.ram[0x0000] = 0xAA;
    mem.ram[0x07FF] = 0xBB;

    assert_eq!(mem.read_byte(0x0000), 0xAA);
    assert_eq!(mem.read_byte(0x07FF), 0xBB);

    //read the last byte of ram and wrap around to the first
    assert_eq!(mem.read_word(0x07FF), 0xAABB);

    //mirrors
    assert_eq!(mem.read_byte(0x0000 + 0x0800), 0xAA);
    assert_eq!(mem.read_byte(0x07FF + 0x0800), 0xBB);
    assert_eq!(mem.read_word(0x07FF + 0x0800), 0xAABB);

    assert_eq!(mem.read_byte(0x0000 + 0x1000), 0xAA);
    assert_eq!(mem.read_byte(0x07FF + 0x1000), 0xBB);
    assert_eq!(mem.read_word(0x07FF + 0x1000), 0xAABB);

    assert_eq!(mem.read_byte(0x0000 + 0x1800), 0xAA);
    assert_eq!(mem.read_byte(0x07FF + 0x1800), 0xBB);

    //this should read the last byte of RAM, wrap around, and read the first ppu register
    //TODO fix this test once I can set up a test ppu
    assert_eq!(mem.read_word(0x07FF + 0x1800), 0x11BB);
}

//TODO fix this whole test, ppu values are hardcoded in mem right now which is not ideal
#[test]
fn mem_ppu_mirror_test() {
    let mem = get_empty_mem();

    let mut i: u16 = 0x2000;

    while i < 0x4000 {
        assert_eq!(mem.read_byte(i), 0x11);
        assert_eq!(mem.read_byte(i + 1), 0x22);
        assert_eq!(mem.read_byte(i + 2), 0x33);
        assert_eq!(mem.read_byte(i + 3), 0x44);
        assert_eq!(mem.read_byte(i + 4), 0x55);
        assert_eq!(mem.read_byte(i + 5), 0x66);
        assert_eq!(mem.read_byte(i + 6), 0x77);
        assert_eq!(mem.read_byte(i + 7), 0x88);

        i += 0x0008;
    }
}
