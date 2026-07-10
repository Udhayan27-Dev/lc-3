pub mod instruction;
pub mod register;
pub mod vm;


use vm::VM;

pub const MEMORY_SIZE:usize = std::u16::MAX as usize;

pub fn exec_prog(vm: &mut VM){
    while vm.registers.pc < MEMORY_SIZE as u16 {
        let instr = vm.read_mem(vm.registers.pc);
        vm.registers.pc += 1;
        
    }
}

pub mod hardware;
use hardware::vm::VM;

extern crate termios;
use termios::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(long)]
    print_asm: bool,
}

fn main() {
    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).unwrap();
}