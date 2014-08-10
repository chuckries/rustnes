use std::io::IoResult;

pub struct Mem;

impl Mem {
    pub fn new() -> Mem {
        Mem
    }
}

impl Reader for Mem {
    fn read(&mut self, buf: &mut[u8]) -> IoResult<uint> {
        return Ok(buf.len());
    }
}
