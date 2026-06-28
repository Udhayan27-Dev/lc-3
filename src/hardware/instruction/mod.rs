use super::vm::VM;

use std::{io::{Read,Write},process};

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
