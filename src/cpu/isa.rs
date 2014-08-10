pub struct Instruction(Instr, AddressMode);

impl Instruction {
    pub fn new(opcode: u8) -> Option<Instruction> {
        decode(opcode)
    }

    pub fn run<T: Reader>(&self, mem: &T) {

    }
}

//so this macro doesn't actually work, but leave it in for now
//this could be used to expand out alu instructions easily in the match statement
macro_rules! alu_inst(
    ($instr:ident $imm:ident $zp:ident $zpx:ident $abs:ident $absx:ident $absy:ident $indx:ident $indy:ident) => (
        $imm => ($instr, IMM),
        $zp => ($instr, ZP),
        $zpx => ($instr, ZPX),
        $abs => ($instr, ABS),
        $absx => ($instr, ABSX),
        $absy => ($instr, ABSY),
        $indx => ($instr, INDX),
        $indy => ($instr, INDY),
    );
)

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

            _ => (INSTR_NONE, ADDRESS_MODE_NONE)
        };

    match (instr, mode) {
        (INSTR_NONE, ADDRESS_MODE_NONE) => None,
        (x, y) => Some(Instruction(x, y))
    }
}

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
