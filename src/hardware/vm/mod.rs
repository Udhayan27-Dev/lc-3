const MEMORY_SIZE: usize = u16::MAX as usize;

use std::io::Read;

use super::register::Registers;

pub struct VM {
    pub memory: [u16; MEMORY_SIZE],
    pub registers: Registers,
}

pub enum MemmoryMappedReg{
    //keyboard status register,it is a Memory Mapped I/O
    Kbsr = 0xFE00,
    //keyboard data register
    Kbdr = 0xFE02,
}

impl VM{
    pub fn new() -> VM {
        VM {
            memory: [0; MEMORY_SIZE],
            registers: Registers::new(),
        }
    }

    pub fn write_mem(&mut self, address: usize, value: u16) {
        self.memory[address] = value;
    }

    pub fn read_mem(&mut self, address: u16) -> u16 {
        self.memory[address as usize]
    }

    fn handle_keyboard(&mut self){
        let mut buffer = [0;1];
        std::io::stdin().read_exact(&mut buffer).unwrap();
        if buffer[0] != 0 {
            self.write_mem(MemmoryMappedReg::Kbsr as usize, 1 << 15);
            self.write_mem(MemmoryMappedReg::Kbdr as usize, buffer[0] as u16);
        }else{
            self.write_mem(MemmoryMappedReg::Kbsr as usize, 0);
        }
    }

}
