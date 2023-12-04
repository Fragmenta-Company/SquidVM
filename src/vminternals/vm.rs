// use log::debug;
use crate::vminternals::{Immediates, VMStack};

pub struct VMStarter<'a> {
    pc: usize,
    instruction: i32,
    instructions: &'a [i32],
    stack: VMStack,
    data: Immediates,
    pub running: bool,
}

impl<'a> VMStarter<'a> {
    pub fn new() -> VMStarter<'a> {
        VMStarter {
            stack: VMStack::new(),
            pc: 0x00,
            instruction: 0x00,
            data: Immediates::Integer(10),
            running: true,
            instructions: &[],
        }
    }

    pub fn get_length(&mut self) -> usize {
        self.stack.get_length()
    }

    pub fn interpreter(&mut self, instructions: &'a [i32], data: &[Immediates]) {
        self.instructions = instructions;

        while self.pc < self.instructions.len() {
            let instruction = self.instructions[self.pc];
            self.data = data[self.pc].clone();
            self.instruction = instruction;
            self.instructor(instruction);
            self.pc += 1;
            println!("{}", self.pc);
        }

        if self.pc > instructions.len() {
            panic!("[ PROGRAM COUNTER OUT OF RANGE ]");
        }
    }

    pub fn push(&mut self, data: Immediates) {
        self.stack.push(data);
    }

    pub fn pop(&mut self) -> Immediates {
        self.stack.pop()
    }

    pub fn check_empty(&mut self) -> bool {
        self.stack.check_empty()
    }

    pub fn instructor(&mut self, instruction: i32) {
        match instruction {
            0x00 => {
                println!("[ HALT ]");
                self.running = false;
                return;
            }
            0x01 => {
                println!("[ iADD ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Immediates::Integer(v1a), Immediates::Integer(v2a)) = (v1, v2) {
                    println!("{} {}", v1a, v2a);

                    println!("{}", v1a + v2a);
                    self.stack.push(Immediates::Integer(v1a + v2a));
                    return;
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            0x02 => {
                println!("[ iSUB ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Immediates::Integer(v1a), Immediates::Integer(v2a)) = (v1, v2) {
                    println!("{}", v1a - v2a);
                    self.stack.push(Immediates::Integer(v1a - v2a));
                    return;
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            0x03 => {
                println!("[ iMUL ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Immediates::Integer(v1a), Immediates::Integer(v2a)) = (v1, v2) {
                    println!("{}", v1a * v2a);
                    self.stack.push(Immediates::Integer(v1a * v2a));
                    return;
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            0x04 => {
                println!("[ iDVD ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Immediates::Integer(v1a), Immediates::Integer(v2a)) = (v1, v2) {
                    if (v1a / v2a) as f64 == (v1a as f64 / v2a as f64) {
                        self.stack.push(Immediates::Integer(v1a / v2a));
                    } else {
                        self.stack.push(Immediates::Float(v1a as f64 / v2a as f64));
                    }

                    return;
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            0x05 => {
                println!("[ FiDVD ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Immediates::Integer(v1a), Immediates::Integer(v2a)) = (v1, v2) {
                    self.stack.push(Immediates::Integer(v1a / v2a));

                    return;
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            0x06 => {
                println!("[ fADD ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Immediates::Float(v1a), Immediates::Float(v2a)) = (v1, v2) {
                    println!("{}", v1a + v2a);

                    self.stack.push(Immediates::Float(v1a + v2a));

                    return;
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            0x07 => {
                println!("[ fSUB ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Immediates::Float(v1a), Immediates::Float(v2a)) = (v1, v2) {
                    println!("{}", v1a - v2a);

                    self.stack.push(Immediates::Float(v1a - v2a));

                    return;
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            0x08 => {
                println!("[ fMUL ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Immediates::Float(v1a), Immediates::Float(v2a)) = (v1, v2) {
                    println!("{}", v1a * v2a);

                    self.stack.push(Immediates::Float(v1a * v2a));

                    return;
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            0x09 => {
                println!("[ fDVD ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Immediates::Float(v1a), Immediates::Float(v2a)) = (v1, v2) {
                    println!("{}", v1a / v2a);

                    self.stack.push(Immediates::Float(v1a / v2a));

                    return;
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            0x0A => {
                println!("[ PDTS ]");

                let pdts = &self.data;

                // if let Immediates::Integer(i) = pdts {
                //
                //     println!("{}", i);
                //
                // }

                self.stack.push(pdts.clone());

                return;

                // println!("{}", self.get_length());
            }
            0x0B => {
                println!("[ PDFS ]");

                self.data = self.stack.pop();

                return;
            }
            0x0C => {
                println!("[ JMPFD ]");

                if let Immediates::Integer(i) = self.data {
                    self.pc = i as usize;

                    return;
                } else {
                    panic!("[ WRONG ADDRESS ]");
                }
            }
            0x0D => {
                println!("[ JMPFS ]");

                if let Immediates::Integer(i) = self.pop() {
                    self.pc = i as usize;

                    return;
                } else {
                    panic!("[ WRONG ADDRESS ]");
                }
            }
            0x0E => {
                println!("[ PRTFS ]");

                if let Immediates::String(s) = self.stack.pop() {
                    println!("{s}");
                } else {
                    panic!("[ NO STRING ]");
                }
            }
            0x0F => {
                println!("[ PRTFD ]");

                if let Immediates::String(s) = self.data.clone() {
                    println!("{s}");
                } else {
                    panic!("[ NO STRING ]");
                }
            }
            _ => {
                panic!("[ UNKNOWN INSTRUCTION ]")
            }
        }
    }
}
