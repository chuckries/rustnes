use cpu::{Cpu};
use cart::{Cart};
use mem::{Mem};

pub struct Nes {
    rom_path: Path,

    //components
    cpu: Cpu,
    cart: Cart,
    mem: Mem,
}

impl Nes {
    pub fn new(rom_path: Path) -> Nes {
        println!("{}", rom_path.display());

        let cart = Cart::new(&rom_path);
        let mem = Mem::new();
        let cpu = Cpu::new();

        Nes { 
            rom_path: rom_path,

            cpu: cpu, 
            cart: cart,
            mem: mem,
        }
    }

    pub fn run(&mut self) {
        self.cpu.run(&self.mem);
    }
}
