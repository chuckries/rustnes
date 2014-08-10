use mem::{Mem};
use self::isa::Instruction;

mod isa;

struct ProcState {
    //processor registers
    pub PC: u16,    //Program Counter
    pub A:  u8,     //Accumulator
    pub X:  u8,     //Index Register X
    pub Y:  u8,     //Index Register Y
    pub SP: u8,     //Stack Pointer
    pub P:  u8,     //Status Register
}

pub struct Cpu {
    state: ProcState,
}

impl Cpu {
    pub fn new() -> Cpu {
        let proc_state = ProcState {
            PC: 0x000,
            A:  0x00,
            X:  0x00,
            Y:  0x00,
            SP: 0x00,
            P:  0x00,
        };

        Cpu { state: proc_state }
    }

    pub fn run(&mut self, mem: &Mem) {
        let &mut state = &self.state;
        let opcode = read(state.PC);
        let instr = Instruction::new(opcode).unwrap();
        instr.run(mem);
    }
}

//simple prototype so I can write around a reader
//not permament
fn read(addr: u16) -> u8 {
    0x00
}
