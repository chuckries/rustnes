use mem::{Mem};
use self::isa::{Instruction, Instr, AddressMode};

mod isa;

#[allow(uppercase_variables)]
struct CpuState {
    //registers
    pub PC: u16,    //Program Counter
    pub A:  u8,     //Accumulator
    pub X:  u8,     //Index Register X
    pub Y:  u8,     //Index Register Y
    pub SP: u8,     //Stack Pointer
    pub P:  u8,     //Status Register
}

pub struct Cpu {
    state: CpuState,

    mem: Mem,
}

impl Cpu {
    pub fn new(mem: Mem) -> Cpu {
        let cpu_state = CpuState {
            PC: 0x000,
            A:  0x00,
            X:  0x00,
            Y:  0x00,
            SP: 0x00,
            P:  0x00,
        };

        Cpu { 
            state: cpu_state,

            mem: mem,
        }
    }

    pub fn run(&mut self) {
    }

    //goal of this function is to execute the next instruction and return the number of cycles
    //elapsed
    pub fn run_instruction(&mut self) -> uint {
        let instr: Instruction = Instruction::new(self.read_pc()).unwrap();

        let m = self.run_instruction_mem_read(instr.address_mode);

        0
    }

    //performs the instruction's memory read phase and returns the value 
    //read from memory
    pub fn run_instruction_mem_read(&mut self, mode: AddressMode) -> u8 {
        /*
        match mode {
            ZP      => 0, //self.mem.read(self.read_pc() as u16),
            ZPX     => 0, //self.mem.read((self.read_pc() + self.state.X) as u16),
            ZPY     => 0, //self.mem.read((self.read_pc() + self.state.Y) as u16),
            ABS     => 0, //self.mem.read(self.read_pc() as u16),
            ABSX    => 0,
            ABSY    => 0,
            IND     => 0,
            IMP     => 0,
            ACC     => 0,
            IMM     => 0,
            REL     => 0,
            INDX    => 0,
            INDY    => 0,

            ADDRESS_MODE_NONE => 0,
        }
        */
        0
    }

    //this function will read the byte at PC and increment PC by 1
    fn read_pc(&mut self) -> u8 {
        let byte = self.mem.read(self.state.PC);
        self.state.PC += 1;
        byte
    }
}
