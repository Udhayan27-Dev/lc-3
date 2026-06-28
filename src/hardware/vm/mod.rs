const MEMORY_SIZE: usize = u16::MAX as usize;

use super::register::Registers;

pub struct VM {
    pub memory: [u16; MEMORY_SIZE],
    pub registers: Registers,
}

impl VM {
    pub fn new() -> VM {
        VM {
            memory: [0; MEMORY_SIZE],
            registers: Registers::new(),
        }
    }

    pub fn write_mem(&mut self,address:usize,value:u16){
        self.memory[address]= value;
    }

    pub fn read_mem(&mut self, address:u16) -> u16{
        self.memory[address as usize]
    }
}
