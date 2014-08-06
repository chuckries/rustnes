extern crate rustnes;

use rustnes::nes::NES;

use std::os;

fn main() {
    let args: Vec<String> = os::args();
    let filename: &str = args[1].as_slice();
    let path: Path = Path::new(filename);
    let nes: NES = NES::new(path);
    nes.print();
}
