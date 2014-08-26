#[macro_escape]

use std::fmt;

use nes::{PrgRom};
use nes::{VAddr};

use self::isa::{
    Instruction, 
    Instr, 
    AddressMode,
};

mod isa;

#[cfg(test)] 
mod test;


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
        self.remove(NZ_FLAG);
        if x == 0 { self.insert(Z_FLAG); }
        else if (x as i8) < 0 { self.insert(N_FLAG); }
    }

    //calculates overflow of a + b = c
    pub fn set_v(&mut self, a: u8, b: u8, c: u8) {
        self.remove(V_FLAG);
        if (((a ^ c) & (b ^ c)) as i8) < 0 { self.insert(V_FLAG); }
    }

    pub fn set_c(&mut self, val: u16) {
        self.remove(C_FLAG);
        if val & !0xFF > 0 { self.insert(C_FLAG); }
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
    pub PC: VAddr,  //Program Counter
    pub A:  u8,     //Accumulator
    pub X:  u8,     //Index Register X
    pub Y:  u8,     //Index Register Y
    pub S:  u8,     //Stack Pointer
    pub P:  CpuFlags,     //Status Register
}

impl CpuState {
    pub fn new() -> CpuState {
        CpuState {
            PC: 0x0000,
            A:  0x00,
            X:  0x00,
            Y:  0x00,
            S:  0xFF,
            P:  CpuFlags::none(),
        }
    }
}

static RAM_SIZE: uint = 0x0800; //2 KB
type Ram = [u8, ..RAM_SIZE];

pub struct Cpu {
    state: CpuState,
    prg_rom: PrgRom,
    ram: Ram,
}

impl Cpu {
    pub fn new(prg_rom: PrgRom) -> Cpu {
        let cpu_state = CpuState::new();

        Cpu { 
            state: cpu_state,
            prg_rom: prg_rom,
            ram: [0u8, ..RAM_SIZE],
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

        if instr.instr == isa::JMP {
            self.state.PC = m_addr;
        } else if instr.instr == isa::JSR {
            let pc = self.state.PC - 1;
            self.push_addr(pc);
            self.state.PC = m_addr;
        } else {
            //get the value referenced by the memory addr
            let m = self.instr_mem_read(m_addr, instr);

            //perform the action of the operation
            let x = self.instr_exec(instr.instr, m);
            
            //write back to ram
            self.instr_mem_write(m_addr, x, instr);
        }

        0
    }

    pub fn instr_decode(&mut self) -> Instruction {
        Instruction::new(self.read_pc_byte())
    }


    fn instr_exec(&mut self, instr: Instr, from_mem: u8) -> u8 {
        let a: u8 = self.state.A;
        let x: u8 = self.state.X;
        let y: u8 = self.state.Y;
        let s: u8 = self.state.S;
        let p: CpuFlags = self.state.P;
        let m: u8 = from_mem;
        let mut out: u8 = 0;
        match instr {
            //Load and Store
            isa::LDA => { self.state.A = m; self.state.P.set_zn(m); }
            isa::LDX => { self.state.X = m; self.state.P.set_zn(m); }
            isa::LDY => { self.state.Y = m; self.state.P.set_zn(m); }
            isa::STA => { out = self.state.A; }
            isa::STX => { out = self.state.X; }
            isa::STY => { out = self.state.Y; }

            //Arithmetic
            isa::ADC => { 
                let val: u16 = (a as u16) + (m as u16) + ((p & C_FLAG).bits as u16);
                self.state.P.set_c(val);
                let val: u8 = val as u8;
                self.state.P.set_v(a, m, val);
                self.state.P.set_zn(val);
                self.state.A = val;
            }
            isa::SBC => {
                let val: u16 = (a as u16) + (!m as u16) + ((p & C_FLAG).bits as u16); //yup, subtraction looks weird. see SBC at http://users.telenet.be/kim1-6502/6502/proman.html#222
                self.state.P.set_c(val);
                let val: u8 = val as u8;
                self.state.P.set_v(val, m, a);
                self.state.P.set_zn(val);
                self.state.A = val;
            }
            isa::INC => { out = m + 1; self.state.P.set_zn(out); }
            isa::INX => { self.state.X += 1; self.state.P.set_zn(self.state.X); }
            isa::INY => { self.state.Y += 1; self.state.P.set_zn(self.state.Y); }
            isa::DEC => { out = m - 1; self.state.P.set_zn(out); }
            isa::DEX => { self.state.X -= 1; self.state.P.set_zn(self.state.X); }
            isa::DEY => { self.state.Y -= 1; self.state.P.set_zn(self.state.Y); }

            //Shift and Rotate
            isa::ASL => { 
                let val: u16 = (m as u16) << 1;
                self.state.P.set_c(val);
                let val: u8 = val as u8;
                self.state.P.set_zn(val);
                out = val;
            }
            isa::LSR => {
                self.state.P.remove(C_FLAG);
                if (m & C_FLAG.bits) > 0 { self.state.P.insert(C_FLAG); }
                out = (m >> 1) & 0x7F;
                self.state.P.set_zn(out);
            }
            isa::ROL => {
                out = (m << 1) | (p.bits & C_FLAG.bits);
                self.state.P.set_c((m as u16) << 1);
                self.state.P.set_zn(out);
            }
            isa::ROR => {
                out = (m >> 1) | if p.contains(C_FLAG) { 0x80 } else { 0x00 };
                self.state.P.remove(C_FLAG);
                if m & 0x01 > 0 { self.state.P.insert(C_FLAG); }
                self.state.P.set_zn(out);
            }

            //Logic
            isa::AND => {
                self.state.A = a & m;
                self.state.P.set_zn(self.state.A);
            }
            isa::ORA => {
                self.state.A = a | m;
                self.state.P.set_zn(self.state.A);
            }
            isa::EOR => {
                self.state.A = a ^ m;
                self.state.P.set_zn(self.state.A);
            }
            
            //Compare and Test Bit
            isa::CMP => {
                let val: u16 = (a as u16) + (!m as u16) + 0x01;
                self.state.P.set_c(val);
                let val: u8 = val as u8;
                self.state.P.set_zn(val);
            }
            isa::CPX => {
                let val: u16 = (x as u16) + (!m as u16) + 0x01;
                self.state.P.set_c(val);
                let val: u8 = val as u8;
                self.state.P.set_zn(val);
            }
            isa::CPY => {
                let val: u16 = (y as u16) + (!m as u16) + 0x01;
                self.state.P.set_c(val);
                let val: u8 = val as u8;
                self.state.P.set_zn(val);
            }
            isa::BIT => {
                self.state.P.remove(Z_FLAG | N_FLAG | V_FLAG);
                if (m as i8) < 0 { self.state.P.insert(N_FLAG); }
                if (m & 0x40) > 0 { self.state.P.insert(V_FLAG); }
                if a & m == 0 { self.state.P.insert(Z_FLAG); }
            }

            //Branch
            isa::BCC => {
                if p.contains(C_FLAG) == false {
                    self.add_pc_rel(m);
                }
            }
            isa::BCS => {
                if p.contains(C_FLAG) {
                    self.add_pc_rel(m);
                }
            }
            isa::BEQ => {
                if p.contains(Z_FLAG) {
                    self.add_pc_rel(m);
                }
            }
            isa::BMI => {
                if p.contains(N_FLAG) {
                    self.add_pc_rel(m);
                }
            }
            isa::BNE => {
                if p.contains(Z_FLAG) == false {
                    self.add_pc_rel(m);
                }
            }
            isa::BPL => {
                if p.contains(N_FLAG) == false {
                    self.add_pc_rel(m);
                }
            }
            isa::BVC => {
                if p.contains(V_FLAG) == false {
                    self.add_pc_rel(m);
                }
            }
            isa::BVS => {
                if p.contains(V_FLAG) {
                    self.add_pc_rel(m);
                }
            }

            //Transfer
            isa::TAX => { self.state.X = a; self.state.P.set_zn(a); }
            isa::TXA => { self.state.A = x; self.state.P.set_zn(x); }
            isa::TAY => { self.state.Y = a; self.state.P.set_zn(a); }
            isa::TYA => { self.state.A = y; self.state.P.set_zn(y); }
            isa::TSX => { self.state.X = s; self.state.P.set_zn(s); }
            isa::TXS => { self.state.S = x; self.state.P.set_zn(x); }

            //Stack
            isa::PHA => { self.push(a); }
            isa::PLA => { self.state.A = self.pop(); self.state.P.set_zn(self.state.A); }
            isa::PHP => { self.push(p.bits); }
            isa::PLP => { self.state.P.bits = self.pop(); }

            //Subroutines and Jump
            //Note: JMP and JSR are implemented in instr_run because they need access to m_addr
            isa::RTS => {
                self.state.PC = self.pop_addr() + 1;
            }
            isa::RTI => {
                self.state.P.bits = self.pop();
                self.state.PC = self.pop_addr();
            }

            //Set and Clear
            isa::SEC => { self.state.P.insert(C_FLAG); }
            isa::SED => { self.state.P.insert(D_FLAG); }
            isa::SEI => { self.state.P.insert(I_FLAG); }
            isa::CLC => { self.state.P.remove(C_FLAG); }
            isa::CLD => { } //no effect on NES
            isa::CLI => { self.state.P.remove(I_FLAG); }
            isa::CLV => { self.state.P.remove(V_FLAG); }

            //Miscellaneous
            isa::NOP => { }
            isa::BRK => {
                let pc = self.state.PC + 1;
                self.push_addr(pc);
                self.push(p.bits | B_FLAG.bits);
                self.state.P.insert(I_FLAG);
                self.state.PC = self.read_addr(0xFFFE);
            }

            _ => { error!("Unimplemented instruction"); }
        }

        out
    }

    pub fn instr_mem_read(&self, addr: VAddr, instr: Instruction) -> u8 {
        let am = instr.address_mode;

        if am == isa::ACC {
            self.state.A
        } else if am == isa::IMM || am == isa::REL {
            addr as u8
        } else {
            match instr.instr {
                isa::ADC | isa::AND | isa::ASL | isa::BIT |
                isa::CMP | isa::CPX | isa::CPY | isa::DEC |
                isa::EOR | isa::INC | 
                isa::LDA | isa::LDX | isa::LDY | isa::LSR |
                isa::ORA | isa::ROL | isa::ROR | isa::SBC 
                => {
                    self.read_byte(addr)
                }
                _ => { 0 }
            }
        }
    }

    pub fn instr_mem_write(&mut self, addr: VAddr, from_exec: u8, instr: Instruction) {
        if instr.address_mode == isa::ACC {
            self.state.A = from_exec;
        } else {
            match instr.instr {
                isa::ASL | isa::DEC | isa::INC | isa::LSR |
                isa::ROL | isa::ROR | isa::STA | isa::STX |
                isa::STY 
                => {
                    self.write_byte(addr, from_exec);
                }
                _ => { }
            }
        }
    }

    //performs the instruction's memory read phase and returns the value 
    //read from memory
    pub fn instr_mem_addr(&mut self, mode: AddressMode) -> VAddr {
        match mode {
            isa::ZP | isa::IMM | isa::REL => { 
                self.read_pc_byte() as VAddr 
            }
            isa::ZPX => { 
                (self.read_pc_byte() + self.state.X) as VAddr
            }
            isa::ZPY => { 
                (self.read_pc_byte() + self.state.Y) as VAddr
            }
            isa::ABS => { 
                self.read_pc_addr()
            }
            isa::ABSX => { 
                self.read_pc_addr() + (self.state.X as VAddr)
            }
            isa::ABSY => { 
                self.read_pc_addr() + (self.state.Y as VAddr)
            }
            isa::IND => {
                let indirect_address: VAddr = self.read_pc_addr();
                self.read_addr(indirect_address)
            }
            isa::IMP | isa::ACC => { //implied and accumulator instr's have no memory reference
                0x0000  
            } 
            isa::INDX=> {
                let indirect_address: VAddr = (self.read_pc_byte() + self.state.X) as VAddr;
                self.read_addr(indirect_address)
            }
            isa::INDY => {
                let indirect_address: VAddr = self.read_pc_byte() as VAddr;
                self.read_addr(indirect_address) + (self.state.Y as VAddr)
            }

            _ => { error!("Impossible match"); 0 }
        }
    }

    //this function will read the byte at PC and increment PC by 1
    fn read_pc_byte(&mut self) -> u8 {
        let byte = self.read_byte(self.state.PC);
        self.state.PC += 1;
        byte
    }

    //this function will read the next two bytes at PC and increment it by 2
    //if (PC) is 0xAA and (PC + 1) is 0xBB, the output of this will be 0xBBAA
    fn read_pc_addr(&mut self) -> VAddr {
        let lo: u8 = self.read_pc_byte();
        let hi: u8 = self.read_pc_byte();

        let word: VAddr = (hi as VAddr) << 8 | (lo as VAddr);
        word
    }

    fn add_pc_rel(&mut self, offset: u8) {
        self.state.PC = (self.state.PC as i16 + ((offset as i8)) as i16) as u16;
    }

    fn push(&mut self, val: u8) {
        let addr: uint = 0x0100 | (self.state.S as uint);
        self.ram[addr] = val;
        self.state.S -= 1;
    }

    fn push_addr(&mut self, addr: VAddr) {
        self.push(((addr >> 8) & 0xFF) as u8);
        self.push((addr & 0xFF) as u8);
    }

    fn pop(&mut self) -> u8 {
        self.state.S += 1;
        let addr: uint = 0x0100 | (self.state.S as uint);
        self.ram[addr]
    } 

    fn pop_addr(&mut self) -> VAddr {
        let lo = self.pop();
        let hi = self.pop();

        let word: VAddr = (hi as VAddr) << 8 | (lo as VAddr);
        word
    }

    pub fn read_addr(&self, virtual_address: VAddr) -> VAddr {
        let lo: u8 = self.read_byte(virtual_address);
        let hi: u8 = self.read_byte(virtual_address + 1);

        let word: VAddr = (hi as VAddr) << 8 | (lo as VAddr);
        word
    }

/// # Memory Map
/// This is from http://nesdev.com/NESDoc.pdf
///  _______________ $10000  _______________
/// | PRG-ROM       |       |               |
/// | Upper Bank    |       |               |
/// |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
/// | PRG-ROM       |       |               |
/// | Lower Bank    |       |               |
/// |_______________| $8000 |_______________|
/// | SRAM          |       | SRAM          |
/// |_______________| $6000 |_______________|
/// | Expansion ROM |       | Expansion ROM |
/// |_______________| $4020 |_______________|
/// | I/O Registers |       |               |
/// |_ _ _ _ _ _ _ _| $4000 |               |
/// | Mirrors       |       | I/O Registers |
/// | $2000-$2007   |       |               |
/// |_ _ _ _ _ _ _ _| $2008 |               |
/// | I/O Registers |       |               |
/// |_______________| $2000 |_______________|
/// | Mirrors       |       |               |
/// | $0000-$07FF   |       |               |
/// |_ _ _ _ _ _ _ _| $0800 |               |
/// | RAM           |       | RAM           |
/// |_ _ _ _ _ _ _ _| $0200 |               |
/// | Stack         |       |               |
/// |_ _ _ _ _ _ _ _| $0100 |               |
/// | Zero Page     |       |               |
/// |_______________| $0000 |_______________|

    //Read a byte from the memory bus
    fn read_byte(&self, virtual_address: VAddr) -> u8 {
        if virtual_address < 0x2000 {
            let address: uint = (virtual_address & 0x07FF) as uint; //Mirrored after 0x0800
            self.ram[address]
        } else if virtual_address < 0x4000 {
            let address: uint = (virtual_address & 0x0007) as uint; //Mirrored after 0x2008
            //TODO calls into PPU at this point
            //TODO several of these registers are read only
            match address {
                0 => { 0x11 } //PPU Control Register 1
                1 => { 0x22 } //PPU Control Register 2
                2 => { 0x33 } //PPU Status Register
                3 => { 0x44 } //SPR-RAM Address Register
                4 => { 0x55 } //SPR-RAM I/O Register
                5 => { 0x66 } //VRAM Address Register 1
                6 => { 0x77 } //VRAM Address Register 2
                7 => { 0x88 } //VRAM I/O Register
                _ => { error!("Impossible"); 0x00 }
            }
        } else if virtual_address < 0x4020 {
            //TODO APU Registers and I/O devices
            0x00
        } else if virtual_address < 0x6000 {
            //TODO Expansion ROM
            0x00
        } else if virtual_address < 0x8000 {
            //TODO SRAM
            0x00
        } 
        
        //TODO I need to implement mapping at some point
        else if virtual_address < 0xC000 {
            let address = (virtual_address & 0x3FFF) as uint;
            self.prg_rom[0][address]
        } else { // if virtual_address <= 0xFFFF
            let address = (virtual_address & 0x3FFF) as uint;
            self.prg_rom[1][address]
        }
    }

    fn write_byte(&mut self, virtual_address: VAddr, val: u8) {
        if virtual_address < 0x2000 {
            let address: uint = (virtual_address as uint) & 0x07FF; //Mirrored after 0x0800
            self.ram[address] = val;
        } else if virtual_address < 0x4000 {
            let address: uint = (virtual_address as uint) & 0x0007; //Mirrorer after 0x2008
            //TODO ppu
            match address {
                0 => { }
                1 => { }
                2 => { error!("PPU Status Register ($2002) is Read Only"); }
                3 => { }
                4 => { }
                5 => { }
                6 => { }
                7 => { }
                _ => { }
            }
        } else if virtual_address < 0x4020 {
            //TODO APU Registers and I/O devices
        } else if virtual_address < 0x6000 {
            //TODO Expansion ROM
        } else if virtual_address < 0x8000 {
            //TODO SRAM
        } else {
            error!("Can't write to PRG-ROM");
        }
    }
}
