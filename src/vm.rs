use std::{
    fs::File,
    io::{self, BufReader},
};

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
enum Flags {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2,
}

#[derive(Debug)]
enum Opcode {
    BR = 0, /* branch */
    ADD,    /* add  */
    LD,     /* load */
    ST,     /* store */
    JSR,    /* jump register */
    AND,    /* bitwise and */
    LDR,    /* load register */
    STR,    /* store register */
    RTI,    /* unused */
    NOT,    /* bitwise not */
    LDI,    /* load indirect */
    STI,    /* store indirect */
    JMP,    /* jump */
    RES,    /* reserved (unused) */
    LEA,    /* load effective address */
    TRAP,   /* execute trap */
}

impl Opcode {
    pub fn get(index: u16) -> Opcode {
        match index {
            0 => Opcode::BR,
            1 => Opcode::ADD,
            2 => Opcode::LD,
            3 => Opcode::ST,
            4 => Opcode::JSR,
            5 => Opcode::AND,
            6 => Opcode::LDR,
            7 => Opcode::RTI,
            8 => Opcode::NOT,
            9 => Opcode::LDI,
            10 => Opcode::STI,
            11 => Opcode::JMP,
            12 => Opcode::RES,
            13 => Opcode::LEA,
            14 => Opcode::TRAP,
            _ => panic!("Invalid index {}. Opcode not found", index),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    PC,
    COND,
    COUNT,
}

pub struct VM {
    pub registers: [u16; 10],
    pub memory: [u16; 1 << 16],
}

impl VM {
    pub fn new() -> Self {
        let mut registers = [0; 10];
        registers[Register::PC as usize] = 0x3000;
        VM {
            registers: registers,
            memory: [0; 1 << 16],
        }
    }

    pub fn run(&mut self) {
        self.registers[Register::COND as usize] = Flags::ZRO as u16;
        loop {
            let instr: u16 = self.read_mem(&self.registers[Register::PC as usize]);
            let op: Opcode = Opcode::get(instr >> 12);
            println!("{:#?}", op);
            break;
        }
    }

    fn read_mem(&self, addr: &u16) -> u16 {
        self.memory[*addr as usize]
    }

    pub fn load_to_memory(&mut self, program_path: &str) -> Result<(), io::Error> {
        let file: File = File::open(program_path)?;
        let mut reader = BufReader::new(file);
        let origin_addr = reader.read_u16::<BigEndian>()?;
        let mut addr = origin_addr as usize;
        loop {
            match reader.read_u16::<BigEndian>() {
                Ok(instr) => {
                    self.memory[addr] = instr;
                    addr += 1;
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        Ok(())
    }
}

mod tests {
    #[test]
    fn pc_value_3000() {
        use crate::vm::Register;
        use crate::vm::VM;
        let vm = VM::new();
        assert_eq!(vm.registers[Register::PC as usize], 0x3000);
    }
}
