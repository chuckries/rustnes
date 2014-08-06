use types::{byte, word};
use cart::Cart;

pub struct NES {
    num: word
}

impl NES {
    pub fn new() -> NES {
        NES{ num: 0xBEEF }
    }

    pub fn print(&self) {
        let cart = Cart::new();
        println!("0x{:X}", self.num);
    }
}
