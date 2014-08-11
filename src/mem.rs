use cart::Cart;

pub struct Mem {
    cart: Cart,
}

impl Mem {
    pub fn new(cart: Cart) -> Mem {
        Mem {
            cart: cart,
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        0u8
    }
}

