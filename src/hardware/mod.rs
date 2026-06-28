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