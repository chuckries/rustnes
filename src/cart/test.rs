#![macro_escape]

use cart::{Cart, CartHeader};
use cart::{PrgRomBank, PrgRom};
use cart::{PRG_ROM_BANK_SIZE, CHR_ROM_BANK_SIZE};
use cart::{PRG_RAM_BANK_SIZE, TRAINER_SIZE};

//TODO move this to cpu test
#[test]
fn cart_read_default_prg_rom_banks_test() {
    let prg_rom = vec![prg_rom_bank!(0xAA), prg_rom_bank!(0xBB)];

    let cart = cart!(prg_rom);
    for i in range(0, cart.prg_rom.len() as u16) {
        assert_eq!(cart.read_from_lower_bank(i), 0xAA as u8);
        assert_eq!(cart.read_from_upper_bank(i), 0xBB as u8);
    }
}
