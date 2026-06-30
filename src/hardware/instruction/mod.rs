use super::vm::VM;

// use std::{io::{Read,Write},process};

pub enum Opcode {
    BR = 0,
    ADD,
    LD,
    ST,
    JSR,
    AND,
    LDR,
    STR,
    RTI,
    NOT,
    LDI,
    STI,
    JMP,
    RES,
    LEA,
    TRAP,    
}

pub fn exec_instr(instr:u16,vm:&mut VM){
    let op_code = get_op_code(&instr);

    match op_code {
        Some(Opcode::ADD) => add(instr, vm),
        // Some(Opcode::AND) => and(),
        // Some(Opcode::NOT) => not(),
        // Some(Opcode::BR) => br(),
        // Some(Opcode::JMP) => jmp(),
        // Some(Opcode::JSR) => jsr(),
        // Some(Opcode::LD) => ld(),
        Some(Opcode::LDI) => ldi(instr,vm),
        // Some(Opcode::LDR) => ldr(),
        // Some(Opcode::LEA) => lea(),
        // Some(Opcode::ST) => st(),
        // Some(Opcode::STI) => sti(),
        // Some(Opcode::STR) => str(),
        // Some(Opcode::TRAP) => trap(),
        _ => {}
    }

    
}

pub fn get_op_code(instr: &u16) -> Option<Opcode> {
    match instr >> 12 {
        0 => Some(Opcode::BR),
        1 => Some(Opcode::ADD),
        2 => Some(Opcode::LD),
        3 => Some(Opcode::ST),
        4 => Some(Opcode::JSR),
        5 => Some(Opcode::AND),
        6 => Some(Opcode::LDR),
        7 => Some(Opcode::STR),
        8 => Some(Opcode::RTI),
        9 => Some(Opcode::NOT),
        10 => Some(Opcode::LDI),
        11 => Some(Opcode::STI),
        12 => Some(Opcode::JMP),
        13 => Some(Opcode::RES),
        14 => Some(Opcode::LEA),
        15 => Some(Opcode::TRAP),
        _ => None,
    }
}

///"ADD" opcode 
/// 15           12 │11        9│8         6│ 5 │4     3│2         0
/// ┌───────────────┼───────────┼───────────┼───┼───────┼───────────┐
/// │      0001     │     DR    │  SR1      │ 0 │  00   │    SR2    │
/// └───────────────┴───────────┴───────────┴───┴───────┴───────────┘
///
///  15           12│11        9│8         6│ 5 │4                 0
/// ┌───────────────┼───────────┼───────────┼───┼───────────────────┐
/// │      0001     │     DR    │  SR1      │ 1 │       IMM5        │
/// └───────────────┴───────────┴───────────┴───┴───────────────────┘

pub fn add(instr:u16,vm:&mut VM){    
    let dr = (instr >> 9) & 0x7; 
    let sr1 = (instr >> 6) & 0x7;
    let imm_flag = (instr >> 5) & 0x1;
    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F,5);
        let val:u32 = imm5 as u32 + vm.registers.get(sr1) as u32;
        vm.registers.update(dr, val as u16);
    } else {
        let sr2 = instr & 0x7;
        let val:u32 = vm.registers.get(sr1) as u32 + vm.registers.get(sr2) as u32;
        vm.registers.update(dr, val as u16);        
    }
    vm.registers.update_cond(dr);
} 

///  "LDI" opcode
///  15           12 11        9 8                                 0
/// ┌───────────────┬───────────┬───────────────────────────────────┐
/// │      1010     │     DR    │               PCOffset9           │
/// └───────────────┴───────────┴───────────────────────────────────┘

pub fn ldi(instr:u16,vm:&mut VM){
    let dr = (instr >> 9) & 0x7;
    let pc_off = sign_extend(instr & 0x1ff, 9);
    let first_read = vm.read_mem(vm.registers.pc + pc_off);
    let res = vm.read_mem(first_read);
    vm.registers.update(dr, res);
    vm.registers.update_cond(dr);
}

pub fn sign_extend(mut x:u16,bit_count:u8) -> u16 {
    if(x >> (bit_count-1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;        
    }
    x   
}
