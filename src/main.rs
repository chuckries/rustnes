extern crate rustnes;

use rustnes::nes::Nes;

use std::os;

fn main() {
    let args: Vec<String> = os::args();

    let filename = 
        if args.len() > 1 { 
            args[1].as_slice() 
        } else {
            "mario.nes"
        };

    let path = Path::new(filename);
    let mut nes = Nes::new(path);

    nes.run();
}
