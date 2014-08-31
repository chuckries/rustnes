use cpu::CpuState;

#[deriving(PartialEq, Show)]
pub struct Instruction {
    pub instr: Instr,
    pub address_mode: AddressMode,
    pub cycles: uint,
}

impl Instruction {
    pub fn new(opcode: u8) -> Instruction {
        match decode(opcode) {
            Some(instr) => { instr }
            None => { println!("Decode failed. Op Code: {:X}", opcode); fail!("FAIL"); }
        }
    }
}

fn decode(opcode: u8) -> Option<Instruction>
{
    let (instr, mode, cycles) =
        match opcode {
            //Load and Store
            0xA9 => (LDA, IMM, 2),
            0xA5 => (LDA, ZP, 3),
            0xB5 => (LDA, ZPX, 4),
            0xAD => (LDA, ABS, 4),
            0xBD => (LDA, ABSX, 4),
            0xB9 => (LDA, ABSY, 4),
            0xA1 => (LDA, INDX, 6),
            0xB1 => (LDA, INDY, 5),

            0xA2 => (LDX, IMM, 2),
            0xA6 => (LDX, ZP, 3),
            0xB6 => (LDX, ZPY, 4),
            0xAE => (LDX, ABS, 4),
            0xBE => (LDX, ABSY, 4),
            
            0xA0 => (LDY, IMM, 2),
            0xA4 => (LDY, ZP, 3),
            0xB4 => (LDY, ZPX, 4),
            0xAC => (LDY, ABS, 4),
            0xBC => (LDY, ABSX, 4),

            0x85 => (STA, ZP, 3),
            0x95 => (STA, ZPX, 4),
            0x8D => (STA, ABS, 4),
            0x9D => (STA, ABSX, 5),
            0x99 => (STA, ABSY, 5),
            0x81 => (STA, INDX, 6),
            0x91 => (STA, INDY, 6),

            0x86 => (STX, ZP, 3),
            0x96 => (STX, ZPY, 4),
            0x8E => (STX, ABS, 4),

            0x84 => (STY, ZP, 3),
            0x94 => (STY, ZPX, 4),
            0x8C => (STY, ABS, 4),

            //Arithmetic
            0x69 => (ADC, IMM, 2),
            0x65 => (ADC, ZP, 3),
            0x75 => (ADC, ZPX, 4),
            0x6D => (ADC, ABS, 4),
            0x7D => (ADC, ABSX, 4),
            0x79 => (ADC, ABSY, 4),
            0x61 => (ADC, INDX, 6),
            0x71 => (ADC, INDY, 5),

            0xE9 => (SBC, IMM, 2),
            0xE5 => (SBC, ZP, 3),
            0xF5 => (SBC, ZPX, 4),
            0xED => (SBC, ABS, 4),
            0xFD => (SBC, ABSX, 4),
            0xF9 => (SBC, ABSY, 4),
            0xE1 => (SBC, INDX, 6),
            0xF1 => (SBC, INDY, 5),

            0xE6 => (INC, ZP, 5),
            0xF6 => (INC, ZPX, 6),
            0xEE => (INC, ABS, 6),
            0xFE => (INC, ABSX, 7),

            0xE8 => (INX, IMP, 2),

            0xC8 => (INY, IMP, 2),

            0xC6 => (DEC, ZP, 5),
            0xD6 => (DEC, ZPX, 6),
            0xCE => (DEC, ABS, 6),
            0xDE => (DEC, ABSX, 7),

            0xCA => (DEX, IMP, 2),

            0x88 => (DEY, IMP, 2),

            //Shift and Rotate
            0x0A => (ASL, ACC, 2),
            0x06 => (ASL, ZP, 5),
            0x16 => (ASL, ZPX, 6),
            0x0E => (ASL, ABS, 6),
            0x1E => (ASL, ABSX, 7),

            0x4A => (LSR, ACC, 2),
            0x46 => (LSR, ZP, 5),
            0x56 => (LSR, ZPX, 6),
            0x4E => (LSR, ABS, 6),
            0x5E => (LSR, ABSX, 7),

            0x2A => (ROL, ACC, 2),
            0x26 => (ROL, ZP, 5),
            0x36 => (ROL, ZPX, 6),
            0x2E => (ROL, ABS, 6),
            0x3E => (ROL, ABSX, 7),

            0x6A => (ROR, ACC, 2),
            0x66 => (ROR, ZP, 5),
            0x76 => (ROR, ZPX, 6),
            0x6E => (ROR, ABS, 6),
            0x7E => (ROR, ABSX, 7),

            //Logic
            0x29 => (AND, IMM, 2),
            0x25 => (AND, ZP, 3),
            0x35 => (AND, ZPX, 4),
            0x2D => (AND, ABS, 4),
            0x3D => (AND, ABSX, 4),
            0x39 => (AND, ABSY, 4),
            0x21 => (AND, INDX, 6),
            0x31 => (AND, INDY, 5),

            0x09 => (ORA, IMM, 2),
            0x05 => (ORA, ZP, 3),
            0x15 => (ORA, ZPX, 4),
            0x0D => (ORA, ABS, 4),
            0x1D => (ORA, ABSX, 4),
            0x19 => (ORA, ABSY, 4),
            0x01 => (ORA, INDX, 6),
            0x11 => (ORA, INDY, 5),

            0x49 => (EOR, IMM, 2),
            0x45 => (EOR, ZP, 3),
            0x55 => (EOR, ZPX, 4),
            0x4D => (EOR, ABS, 4),
            0x5D => (EOR, ABSX, 4),
            0x59 => (EOR, ABSY, 4),
            0x41 => (EOR, INDX, 6),
            0x51 => (EOR, INDY, 5),

            //Compare and Test Bit
            0xC9 => (CMP, IMM, 2),
            0xC5 => (CMP, ZP, 3),
            0xD5 => (CMP, ZPX, 4),
            0xCD => (CMP, ABS, 4),
            0xDD => (CMP, ABSX, 4),
            0xD9 => (CMP, ABSY, 4),
            0xC1 => (CMP, INDX, 6),
            0xD1 => (CMP, INDY, 5),

            0xE0 => (CPX, IMM, 2),
            0xE4 => (CPX, ZP, 3),
            0xEC => (CPX, ABS, 4),

            0xC0 => (CPY, IMM, 2),
            0xC4 => (CPY, ZP, 3),
            0xCC => (CPY, ABS, 4),

            0x24 => (BIT, ZP, 3),
            0x2C => (BIT, ABS, 4),

            //Branch
            0x90 => (BCC, REL, 2),
            
            0xB0 => (BCS, REL, 2),

            0xF0 => (BEQ, REL, 2),

            0x30 => (BMI, REL, 2),

            0xD0 => (BNE, REL, 2),

            0x10 => (BPL, REL, 2),

            0x50 => (BVC, REL, 2),

            0x70 => (BVS, REL, 2),

            //Transfer 
            0xAA => (TAX, IMP, 2),
        
            0x8A => (TXA, IMP, 2),

            0xA8 => (TAY, IMP, 2),

            0x98 => (TYA, IMP, 2),

            0xBA => (TSX, IMP, 2),

            0x9A => (TXS, IMP, 2),

            //Stack
            0x48 => (PHA, IMP, 3),

            0x68 => (PLA, IMP, 4),

            0x08 => (PHP, IMP, 3),

            0x28 => (PLP, IMP, 4),

            //Subroutines and Jump
            0x4C => (JMP, ABS, 3),
            0x6C => (JMP, IND, 5),

            0x20 => (JSR, ABS, 6),

            0x60 => (RTS, IMP, 6),

            0x40 => (RTI, IMP, 6),

            //Set and Clear
            0x38 => (SEC, IMP, 2),

            0xF8 => (SED, IMP, 2),

            0x78 => (SEI, IMP, 2),

            0x18 => (CLC, IMP, 2),

            0xD8 => (CLD, IMP, 2),

            0x58 => (CLI, IMP, 2),

            0xB8 => (CLV, IMP, 2),
            
            //Miscellaneous
            0xEA => (NOP, IMP, 2),

            0x00 => (BRK, IMP, 7),

            _ => (INSTR_NONE, ADDRESS_MODE_NONE, 0)
        };

    match (instr, mode) {
        (INSTR_NONE, ADDRESS_MODE_NONE) => None,
        (x, y) => Some(Instruction { 
            instr: x, 
            address_mode: y,
            cycles: cycles,
        })
    }
}

#[deriving(PartialEq, Show)]
pub enum Instr {
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
}

#[deriving(PartialEq, Show)]
pub enum AddressMode {
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
}
