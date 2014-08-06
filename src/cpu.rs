use mem;
use {word, byte};

struct ProcState {
    //processor registers
    PC: word,    //Program Counter
    A: byte,     //Accumulator
    X: byte,     //Index Register X
    Y: byte,     //Index Register Y
    SP: byte,    //Stack Pointer
    P: byte,     //Status Register
}

pub fn run() {
    mem::read();
    mem::write();
}
