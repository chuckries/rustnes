use cart::{Cart};
use cart::test::*;

use mem::{Mem};
use mem::test::*;

use cpu::{Cpu, CpuState, CpuFlags};
use cpu::isa;

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
    let mut prg_rom_bank = prg_rom_bank!();

    //ADC $AA
    prg_rom_bank[0x0000] = 0x65;
    prg_rom_bank[0x0001] = 0xAA; 

    let mut ram = ram!();
    ram[0xAA] = 0x01;

    let cart = cart!(prg_rom!(prg_rom_bank, prg_rom_bank!()));

    let mem = mem!(cart, ram);

    let mut cpu = get_cpu_with_mem(mem);

    cpu.state.PC = 0x8000;
    cpu.state.A = 0x01;

    let instr = cpu.instr_decode();
    assert_eq!(instr.instr, isa::ADC);
    assert_eq!(instr.address_mode, isa::ZP);
    assert_eq!(cpu.state.PC, 0x8001);

    let m_addr = cpu.instr_mem_addr(instr.address_mode);
    assert_eq!(m_addr, 0xAA);
    assert_eq!(cpu.state.PC, 0x8002);

    let m = cpu.instr_mem_read(m_addr, instr);
    assert_eq!(m, 0x01);
    
    let x = cpu.instr_exec(m, instr);
    assert_eq!(x, 0x00);
    assert_eq!(cpu.state.A, 0x02);
}

#[test]
fn cpu_instr_mem_addr_zp_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);
    prg_rom_bank[0] = 0xAA;

    let mut cpu = get_cpu_with_cart(cart!(prg_rom!(prg_rom_bank, prg_rom_bank!(0xC5))));
    cpu.state.PC = 0x8000;
    
    //$AA
    assert_eq!(cpu.instr_mem_addr(isa::ZP), 0xAA);
    assert_eq!(cpu.state.PC, 0x8001);
}

#[test]
fn cpu_instr_mem_addr_zpx_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);
    prg_rom_bank[0] = 0xF0;
    prg_rom_bank[1] = 0xFF;

    let mut cpu = get_cpu_with_cart(cart!(prg_rom!(prg_rom_bank, prg_rom_bank!(0xC5))));
    cpu.state.PC = 0x8000;
    cpu.state.X = 0x0F;
    
    //$F0,$0F
    assert_eq!(cpu.instr_mem_addr(isa::ZPX), 0x00FF);
    assert_eq!(cpu.state.PC, 0x8001);

    //$FF,$0F
    assert_eq!(cpu.instr_mem_addr(isa::ZPX), 0x000E);
    assert_eq!(cpu.state.PC, 0x8002);
}

#[test]
fn cpu_instr_mem_addr_zpy_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);
    prg_rom_bank[0] = 0xF0;
    prg_rom_bank[1] = 0xFF;

    let mut cpu = get_cpu_with_cart(cart!(prg_rom!(prg_rom_bank, prg_rom_bank!(0xC5))));
    cpu.state.PC = 0x8000;
    cpu.state.Y = 0x0F;
    
    //$F0,$0F
    assert_eq!(cpu.instr_mem_addr(isa::ZPY), 0x00FF);
    assert_eq!(cpu.state.PC, 0x8001);

    //$FF,$0F
    assert_eq!(cpu.instr_mem_addr(isa::ZPY), 0x000E);
    assert_eq!(cpu.state.PC, 0x8002);
}

#[test]
fn cpu_instr_mem_addr_abs_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);
    prg_rom_bank[0] = 0xAA;
    prg_rom_bank[1] = 0xBB;

    let mut cpu = get_cpu_with_cart(cart!(prg_rom!(prg_rom_bank, prg_rom_bank!(0xC5))));
    cpu.state.PC = 0x8000;
    
    //$BBAA
    assert_eq!(cpu.instr_mem_addr(isa::ABS), 0xBBAA);
    assert_eq!(cpu.state.PC, 0x8002);
}

#[test]
fn cpu_instr_mem_addr_absx_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);
    prg_rom_bank[0] = 0xAA;
    prg_rom_bank[1] = 0xBB;
    prg_rom_bank[2] = 0xFF;
    prg_rom_bank[3] = 0xFF;

    let mut cpu = get_cpu_with_cart(cart!(prg_rom!(prg_rom_bank, prg_rom_bank!(0xC5))));
    cpu.state.PC = 0x8000;
    cpu.state.X = 2;
    
    //$BBAA,$02
    assert_eq!(cpu.instr_mem_addr(isa::ABSX), 0xBBAC);
    assert_eq!(cpu.state.PC, 0x8002);

    //$FFFF,$02
    assert_eq!(cpu.instr_mem_addr(isa::ABSX), 0x0001);
    assert_eq!(cpu.state.PC, 0x8004);
}

#[test]
fn cpu_instr_mem_addr_absy_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);
    prg_rom_bank[0] = 0xAA;
    prg_rom_bank[1] = 0xBB;
    prg_rom_bank[2] = 0xFF;
    prg_rom_bank[3] = 0xFF;

    let mut cpu = get_cpu_with_cart(cart!(prg_rom!(prg_rom_bank, prg_rom_bank!())));
    cpu.state.PC = 0x8000;
    cpu.state.Y = 2;
    
    //$BBAA,$02
    assert_eq!(cpu.instr_mem_addr(isa::ABSY), 0xBBAC);
    assert_eq!(cpu.state.PC, 0x8002);

    //$FFFF,$02
    assert_eq!(cpu.instr_mem_addr(isa::ABSY), 0x0001);
    assert_eq!(cpu.state.PC, 0x8004);
}

#[test]
fn cpu_instr_mem_addr_ind_test() {
    let mut prg_rom_bank_0 = prg_rom_bank!(0xC5);
    //$80AA in lower bank
    prg_rom_bank_0[0] = 0xAA;
    prg_rom_bank_0[1] = 0x80;
    //$C0AA in upper bank
    prg_rom_bank_0[2] = 0xAA;
    prg_rom_bank_0[3] = 0xC0;

    //$DDCC
    prg_rom_bank_0[0x00AA] = 0xCC;
    prg_rom_bank_0[0x00AB] = 0xDD;

    let mut prg_rom_bank_1 = prg_rom_bank!(0xC5);
    //$FFEE
    prg_rom_bank_1[0x00AA] = 0xEE;
    prg_rom_bank_1[0x00AB] = 0xFF;

    let mut cpu = get_cpu_with_cart(cart!(prg_rom!(prg_rom_bank_0, prg_rom_bank_1)));
    cpu.state.PC = 0x8000;
    
    //[ $CC ] $80AA
    //[ $DD ] $80AB
    //($80AA)
    assert_eq!(cpu.instr_mem_addr(isa::IND), 0xDDCC);
    assert_eq!(cpu.state.PC, 0x8002);

    //[ $EE ] $C0AA
    //[ $FF ] $C0AB
    //($C0AA)
    assert_eq!(cpu.instr_mem_addr(isa::IND), 0xFFEE);
    assert_eq!(cpu.state.PC, 0x8004);
}

#[test]
fn cpu_instr_mem_addr_imp_test() {
    let mut cpu = get_cpu_with_cart(cart!(prg_rom!()));

    assert_eq!(cpu.instr_mem_addr(isa::IMP), 0x0000);
    assert_eq!(cpu.state.PC, 0x0000);
}

#[test]
fn cpu_instr_mem_addr_acc_test() {
    let mut cpu = get_cpu_with_cart(cart!(prg_rom!()));

    assert_eq!(cpu.instr_mem_addr(isa::ACC), 0x0000);
    assert_eq!(cpu.state.PC, 0x0000);
}

#[test]
fn cpu_instr_mem_addr_imm_test() {
    let mut cpu = get_cpu_with_cart(cart!(prg_rom!()));

    assert_eq!(cpu.instr_mem_addr(isa::IMM), 0x0000);
    assert_eq!(cpu.state.PC, 0x0000);
}

#[test]
fn cpu_instr_mem_addr_rel_test() {
    let mut cpu = get_cpu_with_cart(cart!(prg_rom!()));

    assert_eq!(cpu.instr_mem_addr(isa::REL), 0x0000);
    assert_eq!(cpu.state.PC, 0x0000);
}

#[test]
fn cpu_instr_mem_addr_indx_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);
    prg_rom_bank[0] = 0xAA;
    prg_rom_bank[1] = 0xFF;

    let mut ram = ram!();
    ram[0xAC] = 0xBB;
    ram[0xAD] = 0xCC;

    ram[0x01] = 0xDD;
    ram[0x02] = 0xEE;

    let cart = cart!(prg_rom!(prg_rom_bank, prg_rom_bank!(0xC5)));
    let mem = mem!(cart, ram);

    let mut cpu = get_cpu_with_mem(mem);
    cpu.state.PC = 0x8000;
    cpu.state.X = 0x02;

    //($AA,$02)
    assert_eq!(cpu.instr_mem_addr(isa::INDX), 0xCCBB);
    assert_eq!(cpu.state.PC, 0x8001);

    //($FF,$02)
    assert_eq!(cpu.instr_mem_addr(isa::INDX), 0xEEDD);
    assert_eq!(cpu.state.PC, 0x8002);
}

#[test]
fn cpu_instr_mem_addr_indy_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);
    prg_rom_bank[0] = 0xAA;
    prg_rom_bank[1] = 0xBB;
    prg_rom_bank[2] = 0xCC;

    let mut ram = ram!();
    ram[0xAA] = 0xBB;
    ram[0xAB] = 0xCC;

    ram[0xBB] = 0xDD;
    ram[0xBC] = 0xEE;

    ram[0xCC] = 0xFF;
    ram[0xCD] = 0xFF;

    let cart = cart!(prg_rom!(prg_rom_bank, prg_rom_bank!(0xC5)));
    let mem = mem!(cart, ram);

    let mut cpu = get_cpu_with_mem(mem);
    cpu.state.PC = 0x8000;
    cpu.state.Y = 0x02;

    // [ $BB ] $00AA 
    // [ $CC ] $00AB 
    //($AA),$02
    assert_eq!(cpu.instr_mem_addr(isa::INDY), 0xCCBD);
    assert_eq!(cpu.state.PC, 0x8001);

    // [ $DD ] $00BB
    // [ $EE ] $00BC
    //($BB),$02
    assert_eq!(cpu.instr_mem_addr(isa::INDY), 0xEEDF);
    assert_eq!(cpu.state.PC, 0x8002);

    // [ $FF ] $00CC
    // [ $FF ] $00CD
    //($CC),$02
    assert_eq!(cpu.instr_mem_addr(isa::INDY), 0x0001);
    assert_eq!(cpu.state.PC, 0x8003);
}
