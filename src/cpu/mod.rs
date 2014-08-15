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
        static C_FLAG = 0b00000001,
        static Z_FLAG = 0b00000010,
        static I_FLAG = 0b00000100,
        static D_FLAG = 0b00001000, //unused, always on
        static B_FLAG = 0b00010000,
        static X_FLAG = 0b00100000, //unused, always on
        static V_FLAG = 0b01000000,
        static N_FLAG = 0b10000000,

        static NZ_FLAG     = N_FLAG.bits | Z_FLAG.bits,
        static NVZC_FLAG   = N_FLAG.bits | V_FLAG.bits | Z_FLAG.bits | C_FLAG.bits,
        static NZC_FLAG    = N_FLAG.bits | Z_FLAG.bits | C_FLAG.bits,
        static NV_FLAG     = N_FLAG.bits | V_FLAG.bits,

        static DX_FLAG     = D_FLAG.bits | X_FLAG.bits
    }
)

impl CpuFlags {
    pub fn set_zn(&mut self, x: u8) {
        if x == 0 { self.insert(Z_FLAG); }
        else if (x as i8) < 0 { self.insert(N_FLAG); }
    }

    pub fn clear(&mut self) {
        self.bits = DX_FLAG.bits;
    }

    pub fn none() -> CpuFlags {
        DX_FLAG
    }
}

impl fmt::Show for CpuFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.bits)
    }
}

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
            A:  0x00,
            X:  0x00,
            Y:  0x00,
            SP: 0x00,
            P:  CpuFlags::none(),
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
    pub fn instr_run(&mut self) -> uint {
        let instr = self.instr_decode();

        //get the memory address referenced by this instr
        let m_addr = self.instr_mem_addr(instr.address_mode);

        //get the value referenced by the memory addr
        let m = self.instr_mem_read(m_addr, instr);

        //perform the action of the operation
        let x = self.instr_exec(m, instr);
        
        //write back to ram
        self.instr_mem_write(m_addr, x, instr);

        0
    }

    pub fn instr_decode(&mut self) -> Instruction {
        Instruction::new(self.read_pc_byte())
    }


    pub fn instr_exec(&mut self, from_mem: u8, instr: Instruction) -> u8 {
        let a: u8 = self.state.A;
        let x: u8 = self.state.X;
        let y: u8 = self.state.Y;
        let m: u8 = match instr.address_mode {
            isa::IMM | isa::REL    
                => { self.read_pc_byte() }
            _   => { from_mem }
        };
        let out: u8 = 0;
        match instr.instr {
            isa::ADC => { // A + M + C -> A and C
                let val: u16 = (a as u16) + (m as u16) + (self.state.P & C_FLAG).bits as u16;
                self.state.P.remove(NVZC_FLAG);
                if val & !0xFF > 0 { self.state.P.insert(C_FLAG); }
                let val: u8 = val as u8;
                if (((a ^ val) & (m ^ val)) as i8) < 0 { self.state.P.insert(V_FLAG); } //thanks to http://www.opensourceforu.com/2009/03/joy-of-programming-how-to-detect-integer-overflow/
                self.state.P.set_zn(val);
                self.state.A = val;
            }
            isa::SBC => {
                let val: u16 = (a as u16) + (!m as u16) + (self.state.P & C_FLAG).bits as u16; //yup, subtraction looks weird. see SBC at http://users.telenet.be/kim1-6502/6502/proman.html#222
                self.state.P.remove(NVZC_FLAG);
                if val & !0xFF > 0 { self.state.P.insert(C_FLAG); }
                /*
                if ((a ^ m) as i8) > 0 { //this checks to see if the signs of a and m are the same, if the signs are different overflow can't happen
                   if self.state.P.contains(C_FLAG) {

                   }
                }
                */
                let val: u8 = val as u8;
                if (((a ^ val) & (a ^ m)) as i8) < 0 { self.state.P.insert(V_FLAG); }
                self.state.P.set_zn(val);
                self.state.A = val;
            }
            _ => { error!("Unimplemented instruction"); }
        }

        out
    }

    //I wish I could get rid of the mod name...
    pub fn instr_mem_read(&self, addr: u16, instr: Instruction) -> u8 {
        match instr.instr {
            isa::ADC | isa::AND | isa::ASL | isa::BIT |
            isa::CMP | isa::CPX | isa::CPY | isa::DEC |
            isa::EOR | isa::INC | isa::JMP | isa::JSR |
            isa::LDA | isa::LDX | isa::LDY | isa::LSR |
            isa::ORA | isa::ROL | isa::ROR | isa::SBC 
            => {
                self.mem.read_byte(addr)
            }
            _ => { 0 }
        }
    }

    pub fn instr_mem_write(&mut self, addr: u16, from_exec: u8, instr: Instruction) {
        self.mem.write_byte(addr, from_exec);
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
