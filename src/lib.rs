#![crate_name = "rustnes"]

#![feature(macro_rules)]
#![feature(log_syntax)]

pub mod nes;

mod cpu;
mod mem;
pub mod cart;
