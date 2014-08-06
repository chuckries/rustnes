all: driver

driver: nes_library
	rustc -L . driver.rs

nes_library: 
	rustc nes.rs
