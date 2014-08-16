#[macro_escape]

use std::fmt;

use nes::{PrgRom};

use self::isa::{
    Instruction, 
    Instr, 
    AddressMode,
};

mod isa;

#[cfg(test)] 
mod test;

//VAddr represents an NES virtual address
type VAddr = u16;

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
    pub PC: VAddr,    //Program Counter
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
            isa::IMM | isa::REL => { self.read_pc_byte() }
            _                   => { from_mem }
        };
        let mut out: u8 = 0;
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
                let val: u8 = val as u8;
                //since SBC is A - M - !C = result, it's like result + M + !C = A, so overflow can
                //be done with (A ^ result) & (A ^ M) < 0, which is the same idea as ADC
                if (((a ^ val) & (a ^ m)) as i8) < 0 { self.state.P.insert(V_FLAG); } //I found this slick implementation at http://nesdev.com/6502.txt
                self.state.P.set_zn(val);
                self.state.A = val;
            }
            isa::STA => { out = self.state.A; }
            isa::STX => { out = self.state.X; }
            isa::STY => { out = self.state.Y; }
            isa::LDA => { self.state.A = m; self.state.P.set_zn(m); }
            isa::LDX => { self.state.X = m; self.state.P.set_zn(m); }
            isa::LDY => { self.state.Y = m; self.state.P.set_zn(m); }
            _ => { error!("Unimplemented instruction"); }
        }

        out
    }

    //I wish I could get rid of the mod name...
    pub fn instr_mem_read(&self, addr: VAddr, instr: Instruction) -> u8 {
        match instr.instr {
            isa::ADC | isa::AND | isa::ASL | isa::BIT |
            isa::CMP | isa::CPX | isa::CPY | isa::DEC |
            isa::EOR | isa::INC | isa::JMP | isa::JSR |
            isa::LDA | isa::LDX | isa::LDY | isa::LSR |
            isa::ORA | isa::ROL | isa::ROR | isa::SBC 
            => {
                self.read_byte(addr)
            }
            _ => { 0 }
        }
    }

    pub fn instr_mem_write(&mut self, addr: VAddr, from_exec: u8, instr: Instruction) {
        //TODO only allow instructions that write memory
        self.write_byte(addr, from_exec);
    }

    //performs the instruction's memory read phase and returns the value 
    //read from memory
    pub fn instr_mem_addr(&mut self, mode: AddressMode) -> VAddr {
        match mode {
            isa::ZP      => self.read_pc_byte() as VAddr,
            isa::ZPX     => (self.read_pc_byte() + self.state.X) as VAddr,
            isa::ZPY     => (self.read_pc_byte() + self.state.Y) as VAddr,
            isa::ABS     => self.read_pc_word(),
            isa::ABSX    => self.read_pc_word() + (self.state.X as VAddr), 
            isa::ABSY    => self.read_pc_word() + (self.state.Y as VAddr),
            isa::IND     => {
                let indirect_address: VAddr = self.read_pc_word();
                self.read_addr(indirect_address)
            }
            isa::IMP     => 0x0000, //implied, no memory reference
            isa::ACC     => 0x0000, //accumulator, no memory reference
            isa::IMM     => 0x0000, //immediate, pull the bytes somewhere else
            isa::REL     => 0x0000, //relative, pull the bytes somewhere else
            isa::INDX    => {
                let indirect_address: VAddr = (self.read_pc_byte() + self.state.X) as VAddr;
                self.read_addr(indirect_address)
            }
            isa::INDY    => {
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
    fn read_pc_word(&mut self) -> VAddr {
        let lo: u8 = self.read_pc_byte();
        let hi: u8 = self.read_pc_byte();

        let word: VAddr = (hi as VAddr) << 8 | (lo as VAddr);
        word
    }

    /// Read 2 bytes from the memory bus as an VAddr
    pub fn read_addr(&self, virtual_address: VAddr) -> VAddr {
        let lo: u8 = self.read_byte(virtual_address);
        let hi: u8 = self.read_byte(virtual_address + 1);

        let word: VAddr = (hi as VAddr) << 8 | (lo as VAddr);
        word
    }

/// # Memory Map
/// This is from http://nesdev.com/NESDoc.pdf
///  _______________         _______________
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
        println!("Virtual Address: {:X}", virtual_address);
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
