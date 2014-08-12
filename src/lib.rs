#![crate_name = "rustnes"]
#![feature(phase, macro_rules, globs)]

#[phase(plugin, link)] extern crate log;

pub mod nes;

mod cpu;
mod mem;
mod cart;
