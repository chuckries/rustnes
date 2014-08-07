extern crate rustnes;

use rustnes::nes::NES;

use std::os;

fn main() {
    let args: Vec<String> = os::args();

    let filename = 
        if args.len() > 1 { 
            args[1].as_slice() 
        } else {
            "mario.nes"
        };

    let path: Path = Path::new(filename);
    let nes: NES = NES::new(path);
}
