use mem::Mem;
use cpu::CpuState;

pub struct Instruction {
    pub instr: Instr,
    pub address_mode: AddressMode
}

impl Instruction {
    pub fn new(opcode: u8) -> Option<Instruction> {
        decode(opcode)
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
        (x, y) => Some(Instruction { instr: x, address_mode: y })
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

#[cfg(test)]
mod test {

    use cpu::isa::*;

    #[test]
    fn isa_test() {
        let address_mode: AddressMode = ZP;

        let x: u8 = 
            match address_mode {
                ZP => 0,
                ZPX => 0,
                ZPY => 0,
                ABS => 0,
                ABSX => 0,
                ABSY => 0,
                IND => 0,
                IMP => 0,
                ACC => 0,
                IMM => 0,
                REL => 0,
                INDX => 0,
                INDY => 0,

                ADDRESS_MODE_NONE => 0,
            };
    }
}
