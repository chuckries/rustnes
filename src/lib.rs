#![crate_name = "rustnes"]
#![feature(phase, macro_rules, globs)]

#[phase(plugin, link)] extern crate log;

mod cpu;

pub mod nes;

#[cfg(test)]
mod test {
    mod test {
    }
}
