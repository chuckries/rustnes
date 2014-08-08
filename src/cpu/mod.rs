use mem;

mod decode;

struct ProcState {
    //processor registers
    PC: u16,    //Program Counter
    A:  u8,     //Accumulator
    X:  u8,     //Index Register X
    Y:  u8,     //Index Register Y
    SP: u8,     //Stack Pointer
    P:  u8,     //Status Register
}


pub fn run() {
    mem::read();
    mem::write();
}
