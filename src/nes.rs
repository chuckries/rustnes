use cpu::Cpu;
use cart::Cart;

pub struct Nes {
    rom_path: Path,

    //components
    cpu: Cpu,
}

impl Nes {
    pub fn new(rom_path: Path) -> Nes {
        info!("Rom Path: {}", rom_path.display());

        //create a new cart object to store the contents of the ROM file
        //this ends up being owned by mem, so grab any header data we need out
        //of it before giving it away
        info!("Constructing cart");
        let cart = Cart::new(&rom_path);

        //TODO Get things like horizontal/vertical scrolling here
        
        //construct the cpu, passing in mem. This will allow the cpu
        //to call virtual addresses only, and mem will route correctly 
        //behind the scenes
        info!("Constructing cpu");
        let cpu = Cpu::new(cart);

        //hand back a Nes struct. At this point it only has ownership of the Cpu.
        //I'm still fairly unsure how this will look down the road
        //I will likely hide the PPU behind mem, but I will also want access to it here
        Nes { 
            rom_path: rom_path,

            cpu: cpu, 
        }
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
}
