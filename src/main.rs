pub mod hardware;
use std::{fs::File, io::BufReader};

use byteorder::{BigEndian, ReadBytesExt};
use hardware::vm::VM;

extern crate termios;
use structopt::StructOpt;
use termios::*;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    // #[structopt(long)]
    // print_asm: bool, //future feature
}

fn main() {
    let stdin = 0;
    let termios = termios::Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_iflag &= IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON;
    new_termios.c_lflag &= !(ICANON | ECHO);

    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    //initializing the Virtual Machine
    let mut vm = VM::new();
    //Initializing the CLI
    let cli = Cli::from_args();
    //getting the file name in the terminal
    let f = File::open(cli.path).expect("couldn't open file");
    let mut f = BufReader::new(f);
    //fetching the base address to store the program in the memory...this is the first line of code in the bytecode
    let base_address = f.read_u16::<BigEndian>().expect("Error");
    let mut address = base_address as usize;
    //this loop loads the data in the bytecode program into the VM's memory
    loop {
        match f.read_u16::<BigEndian>() {
            Ok(instr) => {
                vm.write_mem(address, instr);
                address += 1;
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    println!("Read complete")
                } else {
                    println!("failed: {}", e);
                }
                break;
            }
        }
    }
    hardware::exec_prog(&mut vm);
    //reset the stdin fd to termios data
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}
