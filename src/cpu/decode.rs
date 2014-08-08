pub struct Intruction(Instr, AddressMode);

pub enum Instr {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,

    INSTR_NONE
}

pub enum AddressMode {
    ZP,     //Zero Page             AND $12
    IZP,    //Indexed ZeroPage      AND $12,X       LDX $12,Y
    ABS,    //Asolute               AND $1234
    IABS,   //Indexed Absolute      AND $1234,X     AND $1234,Y
    IND,    //Indirect              JMP ($1234)
    IMP,    //Implied               CLD             NOP
    ACC,    //Accumulator           ASL             ROL
    IMM,    //Immediate             AND #$12
    REL,    //Relative              BCS *+5
    IIND,   //Indexed Indirect      AND ($12,X)
    INDI,   //Indirect Indexed      AND ($12),Y

    ADDRESS_MODE_NONE
}

pub fn decode(opcode: u8) -> Option<Intruction> {
    let (instr, mode) = decode_impl(opcode);

    match (instr, mode) {
        (INSTR_NONE, ADDRESS_MODE_NONE) => None,
        (x, y) => Some(Intruction(x,y))
    }
}

fn decode_impl(opcode: u8) -> (Instr, AddressMode)
{
    match opcode {
        0x01 => (BRK, IMP),
        _ => (INSTR_NONE, ADDRESS_MODE_NONE)
    }

}
