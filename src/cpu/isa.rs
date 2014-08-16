use cpu::CpuState;

#[deriving(PartialEq, Show)]
pub struct Instruction {
    pub instr: Instr,
    pub address_mode: AddressMode
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
    let (instr, mode) =
        match opcode {
            //Load and Store
            0xA9 => (LDA, IMM),
            0xA5 => (LDA, ZP),
            0xB5 => (LDA, ZPX),
            0xAD => (LDA, ABS),
            0xBD => (LDA, ABSX),
            0xB9 => (LDA, ABSY),
            0xA1 => (LDA, INDX),
            0xB1 => (LDA, INDY),

            0xA2 => (LDX, IMM),
            0xA6 => (LDX, ZP),
            0xB6 => (LDX, ZPY),
            0xAE => (LDX, ABS),
            0xBE => (LDX, ABSY),
            
            0xA0 => (LDY, IMM),
            0xA4 => (LDY, ZP),
            0xB4 => (LDY, ZPX),
            0xAC => (LDY, ABS),
            0xBC => (LDY, ABSX),

            0x85 => (STA, ZP),
            0x95 => (STA, ZPX),
            0x8D => (STA, ABS),
            0x9D => (STA, ABSX),
            0x99 => (STA, ABSY),
            0x81 => (STA, INDX),
            0x91 => (STA, INDY),

            0x86 => (STX, ZP),
            0x96 => (STX, ZPY),
            0x8E => (STX, ABS),

            0x84 => (STY, ZP),
            0x94 => (STY, ZPX),
            0x8C => (STY, ABS),

            //Arithmetic
            0x69 => (ADC, IMM),
            0x65 => (ADC, ZP),
            0x75 => (ADC, ZPX),
            0x6D => (ADC, ABS),
            0x7D => (ADC, ABSX),
            0x79 => (ADC, ABSY),
            0x61 => (ADC, INDX),
            0x71 => (ADC, INDY),

            0xE9 => (SBC, IMM),
            0xE5 => (SBC, ZP),
            0xF5 => (SBC, ZPX),
            0xED => (SBC, ABS),
            0xFD => (SBC, ABSX),
            0xF9 => (SBC, ABSY),
            0xE1 => (SBC, INDX),
            0xF1 => (SBC, INDY),

            0xE6 => (INC, ZP),
            0xF6 => (INC, ZPX),
            0xEE => (INC, ABS),
            0xFE => (INC, ABSX),

            0xE8 => (INX, IMP),

            0xC8 => (INY, IMP),

            0xC6 => (DEC, ZP),
            0xD6 => (DEC, ZPX),
            0xCE => (DEC, ABS),
            0xDE => (DEC, ABSX),

            0xCA => (DEX, IMP),

            0x88 => (DEY, IMP),

            //Shift and Rotate
            0x0A => (ASL, ACC),
            0x06 => (ASL, ZP),
            0x16 => (ASL, ZPX),
            0x0E => (ASL, ABS),
            0x1E => (ASL, ABSX),

            0x4A => (LSR, ACC),
            0x46 => (LSR, ZP),
            0x56 => (LSR, ZPX),
            0x4E => (LSR, ABS),
            0x5E => (LSR, ABSX),

            0x2A => (ROL, ACC),
            0x26 => (ROL, ZP),
            0x36 => (ROL, ZPX),
            0x2E => (ROL, ABS),
            0x3E => (ROL, ABSX),

            0x6A => (ROR, ACC),
            0x66 => (ROR, ZP),
            0x76 => (ROR, ZPX),
            0x6E => (ROR, ABS),
            0x7E => (ROR, ABSX),

            //Logic
            0x29 => (AND, IMM),
            0x25 => (AND, ZP),
            0x35 => (AND, ZPX),
            0x2D => (AND, ABS),
            0x3D => (AND, ABSX),
            0x39 => (AND, ABSY),
            0x21 => (AND, INDX),
            0x31 => (AND, INDY),

            0x09 => (ORA, IMM),
            0x05 => (ORA, ZP),
            0x15 => (ORA, ZPX),
            0x0D => (ORA, ABS),
            0x1D => (ORA, ABSX),
            0x19 => (ORA, ABSY),
            0x01 => (ORA, INDX),
            0x11 => (ORA, INDY),

            0x49 => (EOR, IMM),
            0x45 => (EOR, ZP),
            0x55 => (EOR, ZPX),
            0x4D => (EOR, ABS),
            0x5D => (EOR, ABSX),
            0x59 => (EOR, ABSY),
            0x41 => (EOR, INDX),
            0x51 => (EOR, INDY),

            //Compare and Test Bit
            0xC9 => (CMP, IMM),
            0xC5 => (CMP, ZP),
            0xD5 => (CMP, ZPX),
            0xCD => (CMP, ABS),
            0xDD => (CMP, ABSX),
            0xD9 => (CMP, ABSY),
            0xC1 => (CMP, INDX),
            0xD1 => (CMP, INDY),

            0xE0 => (CPX, IMM),
            0xE4 => (CPX, ZP),
            0xEC => (CPX, ABS),

            0xC0 => (CPY, IMM),
            0xC4 => (CPY, ZP),
            0xCC => (CPY, ABS),

            0x24 => (BIT, ZP),
            0x2C => (BIT, ABS),

            //Branch
            0x90 => (BCC, REL),
            
            0xB0 => (BCS, REL),

            0xF0 => (BEQ, REL),

            0x30 => (BMI, REL),

            0xD0 => (BNE, REL),

            0x10 => (BPL, REL),

            0x50 => (BVC, REL),

            0x70 => (BVS, REL),

            //Transfer 
            0xAA => (TAX, IMP),
        
            0x8A => (TXA, IMP),

            0xA8 => (TAY, IMP),

            0x98 => (TYA, IMP),

            0xBA => (TSX, IMP),

            0x9A => (TXS, IMP),

            //Stack
            0x48 => (PHA, IMP),

            0x68 => (PLA, IMP),

            0x08 => (PHP, IMP),

            0x28 => (PLP, IMP),

            //Subroutines and Jump
            0x4C => (JMP, ABS),
            0x6C => (JMP, IND),

            0x20 => (JSR, ABS),

            0x60 => (RTS, IMP),

            0x40 => (RTI, IMP),

            //Set and Clear
            0x38 => (SEC, IMP),

            0xF8 => (SED, IMP),

            0x78 => (SEI, IMP),

            0x18 => (CLC, IMP),

            0xD8 => (CLD, IMP),

            0x58 => (CLI, IMP),

            0xB8 => (CLV, IMP),
            
            //Miscellaneous
            0xEA => (NOP, IMP),

            0x00 => (BRK, IMP),

            _ => (INSTR_NONE, ADDRESS_MODE_NONE)
        };

    match (instr, mode) {
        (INSTR_NONE, ADDRESS_MODE_NONE) => None,
        (x, y) => Some(Instruction { instr: x, address_mode: y })
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
