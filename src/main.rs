extern crate rustnes;

use rustnes::nes::NES;

fn main() {
    let nes = NES::new();
    nes.print();
}
