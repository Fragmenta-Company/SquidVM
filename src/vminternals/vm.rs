use crate::sqbinreader::FileReader;
// use log::debug;
use crate::vminternals::immediates::Immediates::{
    self, Array, Binary, Boolean, Float, Integer, Null, String as TypeString, UInteger,
};
use crate::vminternals::{VMHeap, VMStack};

pub struct VMStarter {
    pc: usize,
    instruction: u8,
    instructions: Vec<u8>,
    stack: VMStack,
    heap: VMHeap,
    data: Immediates,
    pub running: bool,
}

pub trait GetLength {
    fn get_length(&mut self) -> usize;
}

impl GetLength for VMStarter {
    fn get_length(&mut self) -> usize {
        self.stack.get_length()
    }
}

impl VMStarter {
    pub fn new(heap_size: usize) -> VMStarter {
        VMStarter {
            stack: VMStack::new(),
            heap: VMHeap::new(heap_size),
            pc: 0x00,
            instruction: 0x00,
            data: Null,
            running: true,
            instructions: Vec::new(),
        }
    }

    pub fn interpreter(&mut self, instructions: Vec<u8>, data: &[Immediates]) {
        self.instructions = instructions.clone();

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

    pub fn interpreter2(&mut self, file_reader: FileReader) {
        self.instructions = file_reader.instructions.clone();

        println!("Instructions: {:?}", self.instructions);

        while self.pc < self.instructions.len() {
            let instruction = self.instructions[self.pc];
            self.data = file_reader.data[self.pc].clone();
            self.instruction = instruction;
            self.pc += 1;
            self.instructor(instruction);
            println!("{}", self.pc);
            // println!("Length: {}", self.heap.heap_memory.len());
        }

        if self.pc > file_reader.instructions.clone().len() {
            panic!("[ PROGRAM COUNTER OUT OF RANGE ]");
        }

        self.pc += 1;
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

    pub fn instructor(&mut self, instruction: u8) {
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

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    println!("{} {}", v1a, v2a);

                    println!("{}", v1a + v2a);
                    self.stack.push(Integer(v1a + v2a));
                    return;
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            0x02 => {
                println!("[ iSUB ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    println!("{}", v1a - v2a);
                    self.stack.push(Integer(v1a - v2a));
                    return;
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            0x03 => {
                println!("[ iMUL ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    println!("{}", v1a * v2a);
                    self.stack.push(Integer(v1a * v2a));
                    return;
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            0x04 => {
                println!("[ iDVD ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    if (v1a / v2a) as f64 == (v1a as f64 / v2a as f64) {
                        self.stack.push(Integer(v1a / v2a));
                    } else {
                        self.stack.push(Float(v1a as f64 / v2a as f64));
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

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    self.stack.push(Integer(v1a / v2a));

                    return;
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            0x06 => {
                println!("[ fADD ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    println!("{}", v1a + v2a);

                    self.stack.push(Float(v1a + v2a));

                    return;
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            0x07 => {
                println!("[ fSUB ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    println!("{}", v1a - v2a);

                    self.stack.push(Float(v1a - v2a));

                    return;
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            0x08 => {
                println!("[ fMUL ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    println!("{}", v1a * v2a);

                    self.stack.push(Float(v1a * v2a));

                    return;
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            0x09 => {
                println!("[ fDVD ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    println!("{}", v1a / v2a);

                    self.stack.push(Float(v1a / v2a));

                    return;
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            0x0A => {
                println!("[ PDTS ]");

                let pdts = &self.data;

                // if let Integer(i) = pdts {
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

                if let UInteger(i) = self.data {
                    self.pc = i as usize;

                    return;
                } else {
                    panic!("[ WRONG ADDRESS ]");
                }
            }
            0x0D => {
                println!("[ JMPFS ]");

                if let UInteger(i) = self.pop() {
                    self.pc = i as usize;

                    return;
                } else {
                    panic!("[ WRONG ADDRESS ]");
                }
            }
            0x0E => {
                print!("[ PRTFS ]");

                if let TypeString(s) = self.stack.pop() {
                    println!("{s}");
                } else {
                    panic!("[ NO STRING ]");
                }
            }
            0x0F => {
                println!("[ PRTFD ]");

                if let TypeString(s) = self.data.clone() {
                    print!("{s}");
                } else {
                    panic!("[ NO STRING ]");
                }
            }
            0x10 => {
                println!("[ iExp ]");

                if let (UInteger(v2), Integer(v1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(Integer(v1.pow(v2 as u32)));
                } else {
                    panic!("[ NO INTEGERS ]")
                }
            }
            0x11 => {
                println!("[ fExp ]");

                if let (Float(v2), Float(v1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(Float(v1.powf(v2)));
                } else {
                    panic!("[ NO FLOATS ]")
                }
            }
            0x12 => {
                println!("[ fiExp ]");

                if let (Integer(v2), Float(v1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(Float(v1.powi(v2 as i32)));
                } else {
                    panic!("[ NO FLOATS ]")
                }
            }
            0x13 => {
                println!("[ PRTAFD ]");

                match self.data.clone() {
                    Null => {
                        print!("Null");
                    }
                    Boolean(b) => {
                        print!("{}", b);
                    }
                    UInteger(ui) => {
                        print!("{}", ui);
                    }
                    Integer(i) => {
                        print!("{}", i);
                    }
                    Float(f) => {
                        print!("{}", f);
                    }
                    TypeString(s) => {
                        print!("{}", s);
                    }
                    Binary(bin) => {
                        print!("{:?}", bin);
                    }
                    Array(arr) => {
                        print!("{:?}", arr)
                    }
                }
            }
            0x14 => {
                println!("[ PRTAFS ]");

                match self.stack.pop() {
                    Null => {
                        print!("Null");
                    }
                    Boolean(b) => {
                        print!("{}", b);
                    }
                    UInteger(ui) => {
                        print!("{}", ui);
                    }
                    Integer(i) => {
                        print!("{}", i);
                    }
                    Float(f) => {
                        print!("{}", f);
                    }
                    TypeString(s) => {
                        println!("{}", s);
                    }
                    Binary(bin) => {
                        println!("{:?}", bin);
                    }
                    Array(arr) => {
                        print!("{:?}", arr)
                    }
                }
            }
            0x15 => {
                println!("[ AddVar ]");

                let var_value = self.stack.pop();
                let var_name = self.stack.pop();

                if let UInteger(var_name) = var_name {
                    self.heap.add_var(var_name as usize, var_value)
                } else {
                    panic!("[ INVALID VAR NAME ]");
                }
            }
            0x16 => {
                println!("[ dVFD ]");

                if let UInteger(var_name) = self.data {
                    println!("{:?}", self.heap.get_var(var_name as usize));
                } else {
                    panic!("[ WRONG VARIABLE NAME ]");
                }
            }
            0x17 => {
                println!("[ dVFS ]");

                if let UInteger(var_name) = self.stack.pop() {
                    println!("{:?}", self.heap.get_var(var_name as usize));
                } else {
                    panic!("[ WRONG VARIABLE NAME ]");
                }
            }
            _ => {
                panic!("[ UNKNOWN INSTRUCTION ]")
            }
        }
    }
}
