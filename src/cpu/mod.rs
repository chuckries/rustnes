use mem::{Mem};

//sigh... monster use statement TODO: make this cleaner
//The whole point of the isa mod was to hide this stuff
use self::isa::{
    Instruction, 
    Instr, 
    AddressMode,
    ZP,     //Zero Page             AND $12
    ZPX,    //Indexed ZeroPage X    AND $12,X
    ZPY,    //Indexed ZeroPage Y    LDX $12,Y
    ABS,    //Asolute               AND $1234
    ABSX,   //Indexed Absolute X    AND $1234,X
    ABSY,   //Indexed Absolute Y    AND $1234,Y
    IND,    //Indirect              JMP ($1234)
    IMP,    //Implied               CLD
    ACC,    //Accumulator           ASL
    IMM,    //Immediate             AND #$12
    REL,    //Relative              BCS *+5
    INDX,   //Indexed Indirect      AND ($12,X)
    INDY,   //Indirect Indexed      AND ($12),Y
    ADDRESS_MODE_NONE,

    //Load and Store
    LDA, LDX, LDY, STA, STX, STY,

    //Arithmetic
    ADC, SBC, INC, INX, INY, DEC, DEX, DEY,

    //Shift and Rotate
    ASL, LSR, ROL, ROR,

    //Logic
    AND, ORA, EOR,

    //Compare and Test Bit
    CMP, CPX, CPY, BIT,

    //Bracnh
    BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,

    //Transfer
    TAX, TXA, TAY, TYA, TSX, TXS,

    //Stack
    PHA, PLA, PHP, PLP,

    //Subroutines and Jump
    JMP, JSR, RTS, RTI,

    //Set and Clear
    SEC, SED, SEI, CLC, CLD, CLI, CLV,

    //Miscellaneous
    NOP, BRK,

    //Undefined
    INSTR_NONE,
};

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
            PC: 0x0000,
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
        let instr: Instruction = Instruction::new(self.read_pc_byte()).unwrap();

        //get the memory address referenced by this instr
        let m_addr = self.instr_mem_addr(instr.address_mode);

        0
    }

    pub fn instr_exec(&mut self) -> u8 {
        0
    }

    //performs the instruction's memory read phase and returns the value 
    //read from memory
    pub fn instr_mem_addr(&mut self, mode: AddressMode) -> u16 {
        match mode {
            ZP      => self.read_pc_byte() as u16,
            ZPX     => (self.read_pc_byte() + self.state.X) as u16,
            ZPY     => (self.read_pc_byte() + self.state.Y) as u16,
            ABS     => self.read_pc_word(),
            ABSX    => self.read_pc_word() + (self.state.X as u16), 
            ABSY    => self.read_pc_word() + (self.state.Y as u16),
            IND     => {
                let indirect_address: u16 = self.read_pc_word();
                self.mem.read_word(indirect_address)
            }
            IMP     => 0, //implied, no memory reference
            ACC     => 0, //accumulator, no memory reference
            IMM     => 0, //immediate, pull the bytes somewhere else
            REL     => 0, //relative, pull the bytes somewhere else
            INDX    => {
                let indirect_address: u16 = (self.read_pc_byte() + self.state.X) as u16;
                self.mem.read_word(indirect_address)
            }
            INDY    => {
                let indirect_address: u16 = self.read_pc_byte() as u16;
                self.mem.read_word(indirect_address + (self.state.X as u16))
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
