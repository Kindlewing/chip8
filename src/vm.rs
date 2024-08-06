use crate::prelude::{Flags, Opcode, Register};
use std::fs::File;
use std::io::{self, Read};

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
            match op {
                Opcode::BR => {
                    use Register as R;
                    let offset = self.sign_extend(instr & 0x1FF, 9);
                    let flag = (instr >> 9) & 0x7;
                    if self.registers[R::COND as usize] & flag == 1 {
                        self.registers[R::PC as usize] += offset;
                    }
                }
                Opcode::ADD => {
                    let dr = (instr >> 9) & 0x7;
                    let sr_1 = (instr >> 6) & 0x7;
                    let imm_fl = (instr >> 5) & 0x1;
                    if imm_fl == 1 {
                        let imm_5: u16 = self.sign_extend(instr & 0x1F, 5);
                        self.registers[dr as usize] = self.registers[sr_1 as usize] + imm_5;
                    } else {
                        let r2: u16 = instr & 0x7;
                        self.registers[dr as usize] =
                            self.registers[sr_1 as usize] + self.registers[r2 as usize];
                    }
                    self.update_flags(dr);
                }
            }
        }
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

    fn update_flags(&mut self, register: u16) {
        if self.registers[register as usize] == 0 {
            self.registers[Register::COND as usize] = Flags::ZRO;
        } else if self.registers[register as usize] >> 15 == 1 {
            self.registers[Register::COND as usize] = Flags::NEG;
        } else {
            self.registers[Register::COND as usize] = Flags::POS;
        }
    }

    fn read_mem(&mut self, addr: u16) -> u16 {
        self.memory[addr as usize]
    }

    fn sign_extend(&self, mut x: u16, bit_count: u16) -> u16 {
        if (x >> bit_count - 1) & 1 == 1 {
            x |= 0xFFFF << bit_count;
        }
        x
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
