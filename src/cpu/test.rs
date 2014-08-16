use nes::{PrgRom};
use nes::test::*;

use cpu::{Cpu, CpuState, CpuFlags, Ram, RAM_SIZE};
use cpu::{C_FLAG, Z_FLAG, I_FLAG, D_FLAG, B_FLAG, X_FLAG, V_FLAG, N_FLAG};
use cpu::isa;

fn get_empty_cpu_state() -> CpuState {
    CpuState::new()
}

fn get_empty_ram() -> Ram {
    [0u8, ..RAM_SIZE]
}

fn get_empty_cpu() -> Cpu {
    let state = get_empty_cpu_state();
    let prg_rom = get_empty_prg_rom();
    let ram = [0u8, ..RAM_SIZE];

    Cpu {
        state: state,
        prg_rom: prg_rom,
        ram: ram,
    }
}

fn get_cpu_with_prg_rom(prg_rom: PrgRom) -> Cpu {
    let state = get_empty_cpu_state();
    let ram = [0u8, ..RAM_SIZE];

    Cpu {
        state: state,
        prg_rom: prg_rom,
        ram: ram,
    }
}

fn get_cpu_with_prg_rom_and_ram(prg_rom: PrgRom, ram: Ram) -> Cpu {
    let state = get_empty_cpu_state();

    Cpu {
        state: state,
        prg_rom: prg_rom,
        ram: ram,
    }
}

#[test]
fn cpu_sanity_test() {
    let mut prg_rom_bank = get_initialized_prg_rom_bank(0xC5);

    //ADC $AA
    prg_rom_bank[0x0000] = 0x65;
    prg_rom_bank[0x0001] = 0xAA; 

    let mut prg_rom = vec![prg_rom_bank, get_initialized_prg_rom_bank(0xC5)];

    let mut ram = get_empty_ram();

    ram[0xAA] = 0x01;

    let mut cpu = get_cpu_with_prg_rom_and_ram(prg_rom, ram);

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

/*
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

//TODO cycle counts
#[test]
fn cpu_instr_adc_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);

    //ADC IMM 1 + 1, no carry, no flags set
    prg_rom_bank[0x0000] = 0x69;
    prg_rom_bank[0x0001] = 0x01;

    //ADC IMM 1 + 1, with carry, no flags set
    prg_rom_bank[0x0002] = 0x69;
    prg_rom_bank[0x0003] = 0x01;

    //ADC IMM 0xFF + 2, no carry, carry set
    prg_rom_bank[0x0004] = 0x69;
    prg_rom_bank[0x0005] = 0xFF;

    //ADC IMM 0xFF + 2, with carry, carry set
    prg_rom_bank[0x0006] = 0x69;
    prg_rom_bank[0x0007] = 0xFF;

    //ADC IMM 0x00 + 0, no carry, zero set
    prg_rom_bank[0x0008] = 0x69;
    prg_rom_bank[0x0009] = 0x00;

    //ADC IMM 0xFF + 1, no carry, carry set, zero set
    prg_rom_bank[0x000A] = 0x69;
    prg_rom_bank[0x000B] = 0xFF;

    //ADC IMM 0x7F + 1, no carry, overflow set, negative set
    prg_rom_bank[0x000C] = 0x69;
    prg_rom_bank[0x000D] = 0x7F;

    //ADC IMM 0x80 + 1, no carry, negative set
    prg_rom_bank[0x000E] = 0x69;
    prg_rom_bank[0x000F] = 0x80;

    //ADC IMM 0xFF + 0xFF, no carry, carry set, negative set
    prg_rom_bank[0x0010] = 0x69;
    prg_rom_bank[0x0011] = 0xFF;

    //ADC IMM 0x80 + 0x80, no carry, carry set, zero set, overflow set
    prg_rom_bank[0x0012] = 0x69;
    prg_rom_bank[0x0013] = 0x80;

    //ADC IMM 0x7F + 0x7F, no carry, overflow set, zero set
    prg_rom_bank[0x0014] = 0x69;
    prg_rom_bank[0x0015] = 0x7F;

    //ADC IMM 0x80 + 0xFF, no carry, overflow set, carry set
    prg_rom_bank[0x0016] = 0x69;
    prg_rom_bank[0x0017] = 0xFF;

    //ADC IMM 0x3F + 0x40, with carry, overflow set, negative set
    prg_rom_bank[0x0018] = 0x69;
    prg_rom_bank[0x0019] = 0x40;

    let mem = mem!(cart!(prg_rom!(prg_rom_bank, prg_rom_bank!(0xC5))));

    let mut cpu = get_cpu_with_mem(mem);
    cpu.state.PC = 0x8000;

    //ADC IMM 1 + 1, no carry
    cpu.state.A = 1;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8002);
    assert_eq!(cpu.state.A, 2);
    assert_eq!(cpu.state.P, CpuFlags::none());

    //ADC IMM 1 + 1, with carry
    cpu.state.A = 1;
    cpu.state.P.clear();
    cpu.state.P.insert(C_FLAG);
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8004);
    assert_eq!(cpu.state.A, 3);
    assert_eq!(cpu.state.P, CpuFlags::none());

    //ADC IMM 0xFF + 2, no carry
    cpu.state.A = 2;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8006);
    assert_eq!(cpu.state.A, 1);
    assert_eq!(cpu.state.P, CpuFlags::none() | C_FLAG);

    //ADC IMM 0xFF + 2, no carry
    cpu.state.A = 2;
    cpu.state.P.clear();
    cpu.state.P.insert(C_FLAG);
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8008);
    assert_eq!(cpu.state.A, 2);
    assert_eq!(cpu.state.P, CpuFlags::none() | C_FLAG);

    //ADC IMM 0x00 + 0, no carry, zero set
    cpu.state.A = 0;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x800A);
    assert_eq!(cpu.state.A, 0);
    assert_eq!(cpu.state.P, CpuFlags::none() | Z_FLAG);

    //ADC IMM 0xFF + 1, no carry, carry set, zero set
    cpu.state.A = 1;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x800C);
    assert_eq!(cpu.state.A, 0);
    assert_eq!(cpu.state.P, CpuFlags::none() | C_FLAG | Z_FLAG);

    //ADC IMM 0x7F + 1, no carry, overflow set, negative set
    cpu.state.A = 1;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x800E);
    assert_eq!(cpu.state.A, 0x80);
    assert_eq!(cpu.state.P, CpuFlags::none() | V_FLAG | N_FLAG);

    //ADC IMM 0x80 + 1, no carry, negative set
    cpu.state.A = 1;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8010);
    assert_eq!(cpu.state.A, 0x81);
    assert_eq!(cpu.state.P, CpuFlags::none() | N_FLAG);

    //ADC IMM 0xFF + 0xFF, no carry, carry set, negative set
    cpu.state.A = 0xFF;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8012);
    assert_eq!(cpu.state.A, 0xFE);
    assert_eq!(cpu.state.P, CpuFlags::none() | C_FLAG | N_FLAG);

    //ADC IMM 0x80 + 0x80, no carry, carry set, overflow set
    cpu.state.A = 0x80;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8014);
    assert_eq!(cpu.state.A, 0x00);
    assert_eq!(cpu.state.P, CpuFlags::none() | C_FLAG | V_FLAG | Z_FLAG);

    //ADC IMM 0x7F + 0x7F, no carry, overflow set, zero set
    cpu.state.A = 0x7F;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8016);
    assert_eq!(cpu.state.A, 0xFE);
    assert_eq!(cpu.state.P, CpuFlags::none() | V_FLAG | N_FLAG);

    //ADC IMM 0x80 + 0xFF, no carry, overflow set, carry set
    cpu.state.A = 0x80;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8018);
    assert_eq!(cpu.state.A, 0x7F);
    assert_eq!(cpu.state.P, CpuFlags::none() | V_FLAG | C_FLAG);

    //ADC IMM 0x3F + 0x40, with carry, overflow set, negative set
    cpu.state.A = 0x3F;
    cpu.state.P.clear();
    cpu.state.P.insert(C_FLAG);
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x801A);
    assert_eq!(cpu.state.A, 0x80);
    assert_eq!(cpu.state.P, CpuFlags::none() | V_FLAG | N_FLAG);
}

#[test]
fn cpu_instr_sbc_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);

    //SBC IMM 0x5 - 0x3, with carry, carry set
    prg_rom_bank[0x0000] = 0xE9;
    prg_rom_bank[0x0001] = 0x03;

    //SBC IMM 0x5 - 0x3, no carry, carry set
    prg_rom_bank[0x0002] = 0xE9;
    prg_rom_bank[0x0003] = 0x03;

    //SBC IMM 0x5 - 0x5, with carry, carry set, zero set
    prg_rom_bank[0x0004] = 0xE9;
    prg_rom_bank[0x0005] = 0x05;

    //SBC IMM 0x3 - 0x5, with carry, negative set, 
    prg_rom_bank[0x0006] = 0xE9;
    prg_rom_bank[0x0007] = 0x05;

    //SBC IMM 0x80 - 0x01, with carry, overflow set, carry set
    prg_rom_bank[0x0008] = 0xE9;
    prg_rom_bank[0x0009] = 0x01;

    //SBC IMM 0x7F - 0xFF, with carry, carry set, negative set
    prg_rom_bank[0x000A] = 0xE9;
    prg_rom_bank[0x000B] = 0xFF;

    //SBC IMM 0xC0 - 0x40, no carry, overflow set, carry set
    prg_rom_bank[0x000C] = 0xE9;
    prg_rom_bank[0x000D] = 0x40;

    let mem = mem!(cart!(prg_rom!(prg_rom_bank, prg_rom_bank!(0xC5))));

    let mut cpu = get_cpu_with_mem(mem);
    cpu.state.PC = 0x8000;

    //SBC IMM 0x5 - 0x3, with carry, carry set
    cpu.state.A = 0x05;
    cpu.state.P.clear();
    cpu.state.P.insert(C_FLAG);
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8002);
    assert_eq!(cpu.state.A, 0x02);
    assert_eq!(cpu.state.P, CpuFlags::none() | C_FLAG);

    //SBC IMM 0x5 - 0x3, no carry, carry set
    cpu.state.A = 0x05;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8004);
    assert_eq!(cpu.state.A, 0x01);
    assert_eq!(cpu.state.P, CpuFlags::none() | C_FLAG);

    //SBC IMM 0x5 - 0x5, with carry, carry set, zero set
    cpu.state.A = 0x05;
    cpu.state.P.clear();
    cpu.state.P.insert(C_FLAG);
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8006);
    assert_eq!(cpu.state.A, 0x00);
    assert_eq!(cpu.state.P, CpuFlags::none() | C_FLAG | Z_FLAG);

    //SBC IMM 0x3 - 0x5, with carry, negative set, overflow set
    cpu.state.A = 0x03;
    cpu.state.P.clear();
    cpu.state.P.insert(C_FLAG);
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x8008);
    assert_eq!(cpu.state.A, 0xFE);
    assert_eq!(cpu.state.P, CpuFlags::none() | N_FLAG);

    //SBC IMM 0x80 - 0x01, with carry, overflow set, carry set
    cpu.state.A = 0x80;
    cpu.state.P.clear();
    cpu.state.P.insert(C_FLAG);
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x800A);
    assert_eq!(cpu.state.A, 0x7F);
    assert_eq!(cpu.state.P, CpuFlags::none() | C_FLAG | V_FLAG);

    //SBC IMM 0x7F - 0xFF, with carry, overflow set, negative set
    cpu.state.A = 0x7F;
    cpu.state.P.clear();
    cpu.state.P.insert(C_FLAG);
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x800C);
    assert_eq!(cpu.state.A, 0x80);
    assert_eq!(cpu.state.P, CpuFlags::none() | V_FLAG | N_FLAG);

    //SBC IMM 0xC0 - 0x40, no carry, overflow set, carry set
    cpu.state.A = 0xC0;
    cpu.state.P.clear();
    cpu.instr_run();
    assert_eq!(cpu.state.PC, 0x800E);
    assert_eq!(cpu.state.A, 0x7F);
    assert_eq!(cpu.state.P, CpuFlags::none() | V_FLAG | C_FLAG);
}

#[test]
fn cpu_instr_sta_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);

    //STA ZP 0x00
    prg_rom_bank[0x0000] = 0x85;
    prg_rom_bank[0x0001] = 0x00;

    //STA ZP 0xFF
    prg_rom_bank[0x0002] = 0x85;
    prg_rom_bank[0x0003] = 0xFF;

    let ram = ram!(0xC5);
    let cart = cart!(prg_rom!(prg_rom_bank, prg_rom_bank!(0xC5)));
    let mem = mem!(cart, ram);

    let mut cpu = get_cpu_with_mem(mem);
    cpu.state.PC = 0x8000;

    cpu.state.A = 0x00;
    assert_eq!(cpu.mem.ram[0x00], 0xC5);
    cpu.instr_run();
    assert_eq!(cpu.mem.ram[0x00], 0x00);

    cpu.state.A = 0xFF;
    assert_eq!(cpu.mem.ram[0xFF], 0xC5);
    cpu.instr_run();
    assert_eq!(cpu.mem.ram[0xFF], 0xFF);
}

#[test]
fn cpu_instr_stx_test() {
    let mut prg_rom_bank = prg_rom_bank!(0xC5);

    //STX ZP 0x00
    prg_rom_bank[0x0000] = 0x86;
    prg_rom_bank[0x0001] = 0x00;

    //STX ZP 0xFF
    prg_rom_bank[0x0002] = 0x86;
    prg_rom_bank[0x0003] = 0xFF;

    let ram = ram!(0xC5);
    let cart = cart!(prg_rom!(prg_rom_bank, prg_rom_bank!(0xC5)));
    let mem = mem!(cart, ram);

    let mut cpu = get_cpu_with_mem(mem);
    cpu.state.PC = 0x8000;

    cpu.state.X = 0x00;
    assert_eq!(cpu.mem.ram[0x00], 0xC5);
    cpu.instr_run();
    assert_eq!(cpu.mem.ram[0x00], 0x00);

    cpu.state.X = 0xFF;
    assert_eq!(cpu.mem.ram[0xFF], 0xC5);
    cpu.instr_run();
    assert_eq!(cpu.mem.ram[0xFF], 0xFF);
}

*/
