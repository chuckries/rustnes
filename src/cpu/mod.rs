/// # Status Register (P)
///
///  7 6 5 4 3 2 1 0
///  N V _ B D I Z C
///  | |   | | | | +--- Carry Flag
///  | |   | | | +----- Zero Flag
///  | |   | | +------- Interrupt Disable 
///  | |   | +--------- Decimal Mode (Allows BCD, not implemented on NES)
///  | |   +----------- Break Command
///  | +--------------- Overflow Flag
///  +----------------- Negative Flag

use std::fmt;

use mem::{Mem};

use self::isa::{
    Instruction, 
    Instr, 
    AddressMode,
};

mod isa;

#[cfg(test)]
pub mod test;

bitflags!(
    flags CpuFlags: u8 {
        //flags for setting
        //use these to set bits by or-ing
        static C_SET = 0b00000001,
        static Z_SET = 0b00000010,
        static I_SET = 0b00000100,
        static D_SET = 0b00001000, //unused
        static B_SET = 0b00010000,
        static X_SET = 0b00100000, //unused
        static V_SET = 0b01000000,
        static N_SET = 0b10000000,

        //flags for clear
        //use these to clear bits by and-ing
        static C_CLEAR = 0b11111110,
        static Z_CLEAR = 0b11111101,
        static I_CLEAR = 0b11111011,
        static D_CLEAR = 0b11110111,
        static B_CLEAR = 0b11101111,
        static X_CLEAR = 0b11011111,
        static V_CLEAR = 0b10111111,
        static N_CLEAR = 0b01111111,

        //helpful combos for clearing
        static NZ_CLEAR     = N_CLEAR.bits & Z_CLEAR.bits,
        static NVZC_CLEAR   = N_CLEAR.bits & V_CLEAR.bits & Z_CLEAR.bits & C_CLEAR.bits,
        static NZC_CLEAR    = N_CLEAR.bits & Z_CLEAR.bits & C_CLEAR.bits,
        static NV_CLEAR     = N_CLEAR.bits & V_CLEAR.bits
    }
)

#[allow(uppercase_variables)]
struct CpuState {
    //registers
    pub PC: u16,    //Program Counter
    pub A:  u8,     //Accumulator
    pub X:  u8,     //Index Register X
    pub Y:  u8,     //Index Register Y
    pub SP: u8,     //Stack Pointer
    pub P:  CpuFlags,     //Status Register
}

impl CpuState {
    pub fn new() -> CpuState {
        CpuState {
            PC: 0x0000,
            A: 0x00,
            X: 0x00,
            Y: 0x00,
            SP: 0x00,
            P: CpuFlags::empty(),
        }
    }
}

pub struct Cpu {
    state: CpuState,

    mem: Mem,
}

impl Cpu {
    pub fn new(mem: Mem) -> Cpu {
        let cpu_state = CpuState::new();

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
        let instr = self.instr_decode();

        //get the memory address referenced by this instr
        let m_addr = self.instr_mem_addr(instr.address_mode);

        0
    }

    pub fn instr_exec(&mut self, instr: Instr) -> u8 {
        match instr {
            isa::ADC => {
                0
            }
            _ => { error!("Unimplemented instruction"); 0 }
        }
    }

    pub fn instr_decode(&mut self) -> Instruction {
        let opcode = self.read_pc_byte();
        Instruction::new(opcode)
    }

    //performs the instruction's memory read phase and returns the value 
    //read from memory
    pub fn instr_mem_addr(&mut self, mode: AddressMode) -> u16 {
        match mode {
            isa::ZP      => self.read_pc_byte() as u16,
            isa::ZPX     => (self.read_pc_byte() + self.state.X) as u16,
            isa::ZPY     => (self.read_pc_byte() + self.state.Y) as u16,
            isa::ABS     => self.read_pc_word(),
            isa::ABSX    => self.read_pc_word() + (self.state.X as u16), 
            isa::ABSY    => self.read_pc_word() + (self.state.Y as u16),
            isa::IND     => {
                let indirect_address: u16 = self.read_pc_word();
                self.mem.read_word(indirect_address)
            }
            isa::IMP     => 0x0000, //implied, no memory reference
            isa::ACC     => 0x0000, //accumulator, no memory reference
            isa::IMM     => 0x0000, //immediate, pull the bytes somewhere else
            isa::REL     => 0x0000, //relative, pull the bytes somewhere else
            isa::INDX    => {
                let indirect_address: u16 = (self.read_pc_byte() + self.state.X) as u16;
                self.mem.read_word(indirect_address)
            }
            isa::INDY    => {
                let indirect_address: u16 = self.read_pc_byte() as u16;
                self.mem.read_word(indirect_address) + (self.state.Y as u16)
            }

            _ => { error!("Impossible match"); 0 }
        }
    }

    //this function will read the byte at PC and increment PC by 1
    fn read_pc_byte(&mut self) -> u8 {
        let byte = self.mem.read_byte(self.state.PC);
        self.state.PC += 1;
        byte
    }

    //this function will read the next two bytes at PC and increment it by 2
    //if (PC) is 0xAA and (PC + 1) is 0xBB, the output of this will be 0xBBAA
    fn read_pc_word(&mut self) -> u16 {
        let lo: u8 = self.read_pc_byte();
        let hi: u8 = self.read_pc_byte();

        let word: u16 = (hi as u16) << 8 | (lo as u16);
        word
    }
}
