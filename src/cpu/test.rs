use cpu::{Cpu, CpuState};
use cpu::isa;

use cart::{Cart};
use cart::test::*;

use mem::{Mem};
use mem::test::*;

fn get_empty_cpu_state() -> CpuState {
    CpuState::new()
}

fn get_empty_cpu() -> Cpu {
    let state = get_empty_cpu_state();
    let mem = get_empty_mem();

    Cpu {
        state: state,
        mem: mem,
    }
}

fn get_cpu_with_mem(mem: Mem) -> Cpu {
    let state = get_empty_cpu_state();
    
    Cpu {
        state: state,
        mem: mem,
    }
}

fn get_cpu_with_cart(cart: Cart) -> Cpu {
    let mem = get_mem_with_cart(cart);
    get_cpu_with_mem(mem)
}

#[test]
fn cpu_sanity_test() {
    let mut prg_rom = get_empty_prg_rom();

    //ADC $AA
    prg_rom.get_mut(0)[0x0000] = 0x65;
    prg_rom.get_mut(0)[0x0001] = 0xAA; 

    assert_eq!(prg_rom[0][0x0000], 0x65);
    assert_eq!(prg_rom[0][0x0001], 0xAA);

    let cart = get_cart_with_prg_rom(prg_rom);
    let mut cpu = get_cpu_with_cart(cart);

    cpu.state.PC = 0x8000;

    let instr = cpu.instr_decode();
    assert_eq!(instr.instr, isa::ADC);
    assert_eq!(instr.address_mode, isa::ZP);

    let m_addr = cpu.instr_mem_addr(instr.address_mode);
    assert_eq!(m_addr, 0xAA);
}
