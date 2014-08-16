use cpu::CpuState;

#[deriving(PartialEq, Show)]
pub struct Instruction {
    pub instr: Instr,
    pub address_mode: AddressMode
}

impl Instruction {
    pub fn new(opcode: u8) -> Instruction {
        decode(opcode).unwrap() //TODO error message here with opcode
    }
}

fn decode(opcode: u8) -> Option<Instruction>
{
    let (instr, mode) =
        match opcode {
            0x00 => (BRK, IMP),

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

            0x85 => (STA, ZP),
            0x95 => (STA, ZPX),
            0x80 => (STA, ABS),
            0x90 => (STA, ABSX),
            0x99 => (STA, ABSY),
            0x81 => (STA, INDX),
            0x91 => (STA, INDY),

            0x86 => (STX, ZP),
            0x96 => (STX, ZPY),
            0x8E => (STX, ABS),

            0x84 => (STY, ZP),
            0x94 => (STY, ZPX),
            0x8C => (STY, ABS),

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
