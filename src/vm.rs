use crate::register::Register;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug)]
enum Flags {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum MemMapReg {
    MR_KBSR = 0xFE00, //Keyboard Status Register. 0xFE00 = 65024.
    MR_KBDR = 0xFE02, //Keyboard Data Register. 0xFE02 = 65026.
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
            7 => Opcode::STR,
            8 => Opcode::RTI,
            9 => Opcode::NOT,
            10 => Opcode::LDI,
            11 => Opcode::STI,
            12 => Opcode::JMP,
            13 => Opcode::RES,
            14 => Opcode::LEA,
            15 => Opcode::TRAP,
            _ => panic!("Invalid index {}. Opcode not found", index),
        }
    }
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
            let pc = self.registers[Register::PC as usize];
            let instr: u16 = self.read_mem(pc);
            let op: Opcode = Opcode::get(instr >> 12);
            break;
        }
    }

    fn read_mem(&mut self, addr: u16) -> u16 {
        self.memory[addr as usize]
    }

    pub fn load_to_memory(&mut self, program_path: &str) -> Result<(), io::Error> {
        let mut tmp = Vec::new();
        let mut file: File = File::open(program_path)?;
        file.read_to_end(&mut tmp)?;
        // the first chunk will be the origin
        let mut iter = tmp.chunks(2);
        // We want to fail if value not found
        let pc = iter.next().unwrap();
        // get origin address
        let mut p = (pc[0] as u16) << 8 | pc[1] as u16;
        // store rest of program
        for e in iter {
            self.memory[p as usize] = (e[0] as u16) << 8 | e[1] as u16;
            p += 1;
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
