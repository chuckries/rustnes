#![crate_name = "rustnes"]
#![feature(phase, macro_rules, globs)]

#[phase(plugin, link)] extern crate log;

pub use nes::{Nes};

mod nes;
mod cpu;
mod ppu;

#[cfg(test)]
mod test {
    mod test {
    }
}
