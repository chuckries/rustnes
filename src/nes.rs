use cart::{Cart};

pub struct NES {
    rom_path: Path,

    //components
    cart: Cart
}

impl NES {
    pub fn new(rom: Path) -> NES {
        println!("{}", rom.display());

        let cart = Cart::new(&rom);

        NES{ 
            rom_path: rom,

            cart: cart 
        }

    }
}
