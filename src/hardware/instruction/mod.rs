use std::{io::{self, Read, Write}, process};

use super::vm::VM;



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

pub enum TrapCode{
    ///get character from keyboard
    Getc = 0x20,
    ///output a character
    Out = 0x21,
    ///output a word string
    Puts = 0x22,
    ///input a string
    In = 0x23,
    ///output a byte string
    Putsp = 0x24,
    ///Halt the program
    Halt = 0x25,
}


pub fn exec_instr(instr:u16,vm:&mut VM){
    let op_code = get_op_code(&instr);

    match op_code {
        Some(Opcode::ADD) => add(instr, vm),
        Some(Opcode::AND) => and(instr, vm),
        Some(Opcode::NOT) => not(instr, vm),
        Some(Opcode::BR) => br(instr,vm),
        Some(Opcode::JMP) => jmp(instr,vm),
        Some(Opcode::JSR) => jsr(instr,vm),
        Some(Opcode::LD) => ld(instr,vm),
        Some(Opcode::LDI) => ldi(instr,vm),
        Some(Opcode::LDR) => ldr(instr,vm),
        Some(Opcode::LEA) => lea(instr,vm),
        Some(Opcode::ST) => st(instr,vm),
        Some(Opcode::STI) => sti(instr,vm),
        Some(Opcode::STR) => str(instr,vm),
        Some(Opcode::TRAP) => trap(instr,vm),
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
        let val = imm5.wrapping_add(vm.registers.get(sr1));
        vm.registers.update(dr, val as u16);
    } else {
        let sr2 = instr & 0x7;
        let val = vm.registers.get(sr1).wrapping_add(vm.registers.get(sr2));
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

/// "AND" opcode
/// 15           12 │11        9│8         6│ 5 │4     3│2         0
/// ┌───────────────┼───────────┼───────────┼───┼───────┼───────────┐
/// │      0101     │     DR    │  SR1      │ 0 │  00   │    SR2    │
/// └───────────────┴───────────┴───────────┴───┴───────┴───────────┘

///  15           12│11        9│8         6│ 5 │4                 0
/// ┌───────────────┼───────────┼───────────┼───┼───────────────────┐
/// │      0101     │     DR    │  SR1      │ 1 │       IMM5        │
/// └───────────────┴───────────┴───────────┴───┴───────────────────┘

pub fn and(instr: u16,vm: &mut VM){
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;

    let imm_flag = (instr >> 5) & 0x1;
    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        vm.registers.update(dr, vm.registers.get(sr1) & imm5);
    }
    else {
        let sr2 = instr & 0x7;
        vm.registers.update(dr, vm.registers.get(sr1) & vm.registers.get(sr2));
    }
    vm.registers.update_cond(dr);
}

/// "NOT" opcode
/// 15           12 │11        9│8         6│ 5 │4                 0
/// ┌───────────────┼───────────┼───────────┼───┼───────────────────┐
/// │      1001     │     DR    │     SR    │ 1 │       1111        │
/// └───────────────┴───────────┴───────────┴───┴───────────────────┘
pub fn not(instr: u16,vm:&mut VM){
    let dr = (instr >> 9) & 0x7;
    let sr1 = (instr >> 6) & 0x7;
    vm.registers.update(dr, !vm.registers.get(sr1));
    vm.registers.update_cond(dr);
}

/// "BRANCH" opcode (br is used conditional branching)
/// 15           12 │11 │10 │ 9 │8                                 0
/// ┌───────────────┼───┼───┼───┼───────────────────────────────────┐
/// │      0000     │ N │ Z │ P │             PCOffset9             │
/// └───────────────┴───┴───┴───┴───────────────────────────────────┘
pub fn br(instr:u16,vm:&mut VM){
    let pc_off = sign_extend(instr & 0x1ff,9);
    let cond_flag = instr >> 9 & 0x7;
    if cond_flag & vm.registers.cond != 0 {
        let val = vm.registers.pc.wrapping_add(pc_off);
        vm.registers.pc = val as u16;
    }
}


/// "JUMP" opcode (unconditional)
///  15           12│11        9│8         6│ 5                    0
/// ┌───────────────┼───────────┼───────────┼───────────────────────┐
/// │      1100     │    000    │   BaseR   │       00000           │
/// └───────────────┴───────────┴───────────┴───────────────────────┘
///  15           12│11        9│8         6│ 5                    0
/// ┌───────────────┼───────────┼───────────┼───────────────────────┐
/// │      1100     │    000    │    111    │       00000           │
/// └───────────────┴───────────┴───────────┴───────────────────────┘

pub fn jmp(instr: u16,vm: &mut VM){
    let base = (instr >> 6) & 0x7;
    vm.registers.pc = vm.registers.get(base);
}


/// "JUMP to Subroutine or func" Opcode
///  15           12│11 │10
/// ┌───────────────┼───┼───────────────────────────────────────────┐
/// │      0100     │ 1 │                PCOffset11                 │
/// └───────────────┴───┴───────────────────────────────────────────┘
///  15           12│11 │10    9│8     6│5                         0
/// ┌───────────────┼───┼───────┼───────┼───────────────────────────┐
/// │      0100     │ 0 │   00  │ BaseR │           00000           │
/// └───────────────┴───┴───────┴───────┴───────────────────────────┘

pub fn jsr(instr: u16,vm:&mut VM){
    let base_reg = (instr >> 6) & 0x7;
    let flag = (instr >> 11) &0x1;
    let pc_off = sign_extend(instr & 0x7ff, 11);
    vm.registers.r7 = vm.registers.pc;

    if flag != 0{
        let val = vm.registers.pc.wrapping_add(pc_off);
        vm.registers.pc = val as u16;
    }
    else{
        vm.registers.pc = vm.registers.get(base_reg);
    }
}

///"LOAD" Opcode
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      0010     │     DR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
pub fn ld(instr:u16,vm:&mut VM){
    let dr = (instr >> 9) & 0x7;
    let pc_off = sign_extend(instr & 0x1ff,9);
    let mem = pc_off.wrapping_add(vm.registers.pc);
    let value = vm.read_mem(mem);
    vm.registers.update(dr, value);
    vm.registers.update_cond(dr);
}


/// "LDR" opcode
///  15           12│11        9│8             6│5                 0
/// ┌───────────────┼───────────┼───────────────┼───────────────────┐
/// │      1010     │     DR    │     BaseR     │     PCOffset6     │
/// └───────────────┴───────────┴───────────────┴───────────────────┘
pub fn ldr(instr: u16,vm: &mut VM) {
    let dr = (instr >> 9) & 0x7;
    let base = (instr >> 6) & 0x7;
    let pc_off = sign_extend(instr & 0x3f,6);
    let val = vm.registers.get(base).wrapping_add(pc_off);
    let mem_value = vm.read_mem(val);
    vm.registers.update(dr, mem_value);
    vm.registers.update_cond(dr);
}


/// "LEA"-> load effective address opcode
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      1110     │     DR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
pub fn lea(instr: u16,vm:&mut VM){
    let dr = (instr >> 9) & 0x7;
    let pc_off = sign_extend(instr & 0x1ff, 9);
    let ea = vm.registers.pc.wrapping_add(pc_off);
    vm.registers.update(dr, ea);
    vm.registers.update_cond(dr);    
}

/// "STR" opcode
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      0011     │     SR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
pub fn st(instr: u16,vm:&mut VM){
    let sr = (instr >> 9) & 0x7;
    let pc_off = sign_extend(instr & 0x1ff, 9);
    let addr = vm.registers.pc.wrapping_add(pc_off);
    vm.write_mem(addr as usize, vm.registers.get(sr));
}


/// "STI" Opcode
///  15           12│11        9│8                                 0
/// ┌───────────────┼───────────┼───────────────────────────────────┐
/// │      1011     │     SR    │            PCOffset9              │
/// └───────────────┴───────────┴───────────────────────────────────┘
pub fn sti(instr: u16,vm:&mut VM){
    let sr = (instr >> 9) & 0x7;
    let pc_off = sign_extend(instr & 0x1ff, 9);
    let addr = vm.registers.pc.wrapping_add(pc_off);
    let val = vm.read_mem(addr);
    vm.write_mem(val as usize, vm.registers.get(sr));
    
}


/// 
///  15           12│11        9│8         6│                      0
/// ┌───────────────┼───────────┼───────────┼───────────────────────┐
/// │      0111     │     SR    │   BaseR   │        PCOffset6      │
/// └───────────────┴───────────┴───────────┴───────────────────────┘
pub fn str(instr:u16,vm: &mut VM){
    let sr = (instr >> 9) & 0x7;
    let base_r = (instr >> 6) & 0x7;
    let pc_off = sign_extend(instr & 0x3f, 6);
    let addr = vm.registers.get(base_r).wrapping_add(pc_off);
    vm.write_mem(addr as usize, sr);
}


pub fn trap(instr: u16,vm:&mut VM){
    match instr & 0xff{
        0x20 => {
            //Get character
            let mut buffer = [0;1];
            std::io::stdin().read_exact(&mut buffer).unwrap();
            vm.registers.r0 = buffer[0] as u16;
        }
        0x21 => {
            //write a character
            let c = vm.registers.r0 as u8;
            print!("{}", c as char);
        }
        0x22 => {
            //puts
            let mut index = vm.registers.r0;
            let mut c = vm.read_mem(index);
            while c != 0x0000{
                print!("{}",(c as u8) as char);
                index +=1;
               c = vm.read_mem(index); 
            }
            io::stdout().flush().expect("failed to flush");
        }
        0x23 => {
            //gets
            print!("Enter a character : ");
            io::stdout().flush().expect("failed to flush");
            let char = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as u16)
                .unwrap();
            vm.registers.update(0, char);
        }
        0x24 => {
            //putsp
            let mut index = vm.registers.r0;
            let mut c = vm.read_mem(index);
            while c != 0x0000{
                let c1 = ((c & 0xFF) as u8) as char;
                print!("{}",c1);
                let c2 = ((c >> 8)as u8) as char;
                if c2 != '\0' {
                    print!("{}",c2);
                }
                index += 1;
                c = vm.read_mem(index);
            }
            io::stdout().flush().expect("failed to flush");
        }
        0x25 => {
            //halt process 
            println!("HALT detected");
            io::stdout().flush().expect("failed to flush");
            process::exit(1);
        }
        _ => {
            process::exit(1);
        }
    }
}


pub fn sign_extend(mut x:u16,bit_count:u8) -> u16 {
    if(x >> (bit_count-1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;        
    }
    x   
}
