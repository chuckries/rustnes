use mem::{Mem};
use self::isa::Instruction;

mod isa;

#[allow(uppercase_variables)]
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

    mem: Mem,
}

impl Cpu {
    pub fn new(mem: Mem) -> Cpu {
        let proc_state = ProcState {
            PC: 0x000,
            A:  0x00,
            X:  0x00,
            Y:  0x00,
            SP: 0x00,
            P:  0x00,
        };

        Cpu { 
            state: proc_state,

            mem: mem,
        }
    }

    pub fn run(&mut self) {
        let &mut state = &self.state;
        let opcode = self.mem.read(state.PC);
        let instr = Instruction::new(opcode).unwrap();
        instr.run(&self.mem);
    }
}
