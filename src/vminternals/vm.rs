use crate::instructiondefs::*;
use crate::sqdbinreader::FileReader;
use crate::vminternals::immediates::Immediates::{
    self, Array, Binary, Boolean, Float, Integer, Null, String as TypeString, UInteger, RefPtr
};
use crate::vminternals::{VMHeap, VMRepository, VMStack};

fn print_any(printable: Immediates) {
    match printable {
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
        RefPtr(u) => {
            print!("{u:x}");
        }
    }
}

#[cfg(not(feature = "devkit"))]
/// The **VM's heart**.
/// Contains _instructions_, _data_,
/// _heap_, _stack_, the _program counter_,
/// and all the other stuff for the VM to work.
pub struct VMStarter {
    /// Tells if the VM is running or not.
    pub running: bool,

    /// The program counter.
    /// Contains the instruction pointer.
    pc: usize,

    /// Contains the instruction the VM is using at the moment.
    instruction: u8,

    /// _Is coordinated by the program counter_.
    /// Contains all the instructions the VM will use.
    /// Can be modified at runtime (**WIP**).
    instructions: Vec<u8>,

    /// Contains the data being used at the moment.
    /// In the interpreter function it's changed frequently by the _program counter_.
    data: Immediates,

    /// Contains all the data the VM will use to run the program.
    /// Controlled normally by the program counter.
    data_vault: Vec<Immediates>,

    /// This is the core of the VM
    /// since it's used for almost all instructions.
    /// Normally used for function frames and to store function locals.
    stack: VMStack,

    /// Contains all objects used by the program.
    /// Normally used for dynamic programs that make use of
    /// mutable variables or other objects that can change and/or need to be
    /// stored for a longer time than in the stack.
    heap: VMHeap,

    /// It's used to store pointer for heap values, so it can be used as a global variable storage.
    ///
    /// <p style="color: #FF6E6E;">Warning:</p>
    ///
    /// * All objects used by the repository will **ignored** by the garbage collcetor.
    /// * Only use global variables when they are needed, since they can be a security risk.
    repository: VMRepository,
}

#[cfg(feature = "devkit")]
#[derive(Debug)]
pub struct VMStarter {
    pub running: bool,
    pc: usize,
    instruction: u8,
    instructions: Vec<u8>,
    data: Immediates,
    data_vault: Vec<Immediates>,
    stack: VMStack,
    heap: VMHeap,
    repository: VMRepository,
}

impl VMStarter {
    /// Instantiates the VMStarter struct. Very straight forward.
    pub fn new(heap_size: usize, repository_size: usize) -> VMStarter {
        VMStarter {
            running: true,
            pc: 0x00,
            instruction: 0x00,
            instructions: Vec::new(),
            data: Null,
            data_vault: Vec::new(),
            stack: VMStack::new(),
            heap: VMHeap::new(heap_size),
            repository: VMRepository::new(repository_size),
        }
    }

    /// Gets a FileReader instance and uses it to run the instructor function.
    ///
    /// It will run until the program counter is less than the instructions vector length.
    ///
    /// <p style="color: #FF6E6E;">Warning:</p>
    ///
    /// * Will panic out if the program counter is out of range.
    /// * It always increments by one the program counter after
    /// the interpreter is done executing the program, so it
    /// doesn't run indefinitely if the file was encoded the wrong way.
    pub fn interpreter(&mut self, file_reader: FileReader) {
        self.instructions = file_reader.instructions;
        self.data_vault = file_reader.data;

        dev_print!("Instructions: {:?}", self.instructions);

        while self.pc < self.instructions.len() {
            let instruction = self.instructions[self.pc];
            self.data = self.data_vault[self.pc].clone();
            self.instruction = instruction;
            self.pc += 1;
            self.instructor(instruction);
            dev_print!("{}", self.pc);
            // println!("Length: {}", self.heap.heap_memory.len());
        }

        if self.pc > self.instructions.len() {
            panic!("[ PROGRAM COUNTER OUT OF RANGE ]");
        }

        self.pc += 1;
    }

    /// Contains all the instructions and their implementations.
    /// Receives an instruction and works around it.
    ///
    /// **Panics** if instruction is _unknown_.
    fn instructor(&mut self, instruction: u8) {
        match instruction {
            HALT => {
                dev_print!("[ HALT ]");
                self.running = false;
            }
            I_ADD => {
                dev_print!("[ iADD ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    dev_print!("{} {}", v1a, v2a);

                    dev_print!("{}", v1a + v2a);
                    self.stack.push(Integer(v1a + v2a));
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            I_SUB => {
                dev_print!("[ iSUB ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a - v2a);
                    self.stack.push(Integer(v1a - v2a));
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            I_MUL => {
                dev_print!("[ iMUL ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a * v2a);
                    self.stack.push(Integer(v1a * v2a));
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            I_DVD => {
                dev_print!("[ iDVD ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    if (v1a / v2a) as f64 == (v1a as f64 / v2a as f64) {
                        self.stack.push(Integer(v1a / v2a));
                    } else {
                        self.stack.push(Float(v1a as f64 / v2a as f64));
                    }
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            F_I_DVD => {
                dev_print!("[ FiDVD ]");
                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    self.stack.push(Integer(v1a / v2a));
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            F_ADD => {
                dev_print!("[ fADD ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a + v2a);

                    self.stack.push(Float(v1a + v2a));
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            F_SUB => {
                dev_print!("[ fSUB ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a - v2a);

                    self.stack.push(Float(v1a - v2a));
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            F_MUL => {
                dev_print!("[ fMUL ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a * v2a);

                    self.stack.push(Float(v1a * v2a));
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            F_DVD => {
                dev_print!("[ fDVD ]");

                let v2 = self.stack.pop();
                let v1 = self.stack.pop();

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a / v2a);

                    self.stack.push(Float(v1a / v2a));
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            PDTS => {
                dev_print!("[ PDTS ]");

                let pdts = &self.data;

                self.stack.push(pdts.clone());
            }
            PDFS => {
                dev_print!("[ PDFS ]");

                self.data = self.stack.pop();
            }
            JMPFD => {
                dev_print!("[ JMPFD ]");

                if let UInteger(i) = self.data {
                    self.pc = i as usize;
                } else {
                    panic!("[ WRONG ADDRESS ]");
                }
            }
            JMPFS => {
                dev_print!("[ JMPFS ]");

                if let UInteger(i) = self.stack.pop() {
                    self.pc = i as usize;
                } else {
                    panic!("[ WRONG ADDRESS ]");
                }
            }
            PRTFS => {
                dev_print!("[ PRTFS ]");

                if let TypeString(s) = self.stack.pop() {
                    print!("{s}");
                } else {
                    panic!("[ NO STRING ]");
                }
            }
            PRTFD => {
                dev_print!("[ PRTFD ]");

                if let TypeString(s) = self.data.clone() {
                    print!("{s}");
                } else {
                    panic!("[ NO STRING ]");
                }
            }
            I_EXP => {
                dev_print!("[ iExp ]");

                if let (UInteger(v2), Integer(v1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(Integer(v1.pow(v2 as u32)));
                } else {
                    panic!("[ NO INTEGERS ]")
                }
            }
            F_EXP => {
                dev_print!("[ fExp ]");

                if let (Float(v2), Float(v1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(Float(v1.powf(v2)));
                } else {
                    panic!("[ NO FLOATS ]")
                }
            }
            F_I_EXP => {
                dev_print!("[ fiExp ]");

                if let (Integer(v2), Float(v1)) = (self.stack.pop(), self.stack.pop()) {
                    self.stack.push(Float(v1.powi(v2 as i32)));
                } else {
                    panic!("[ NO FLOATS ]")
                }
            }
            PRTAFD => {
                dev_print!("[ PRTAFD ]");

                print_any(self.data.clone());
            }
            PRTAFS => {
                dev_print!("[ PRTAFS ]");

                print_any(self.stack.pop());
            }
            AVP => {
                dev_print!("[ AVP ]");

                let var_pointer = self.stack.pop();
                let var_name = self.stack.pop();

                if let UInteger(var_name) = var_name {
                    if let UInteger(var_pointer) = var_pointer {
                        self.repository
                            .add_var(var_name as usize, var_pointer as usize)
                    } else {
                        panic!("[ INVALID VAR POINTER ]");
                    }
                } else {
                    panic!("[ INVALID VAR NAME ]");
                }
            }
            D_VFD => {
                dev_print!("[ dVFD ]");

                if let UInteger(var_name) = self.data {
                    println!("Pointer: {}", self.repository.get_var(var_name as usize));
                } else {
                    panic!("[ WRONG VARIABLE NAME ]");
                }
            }
            D_VFS => {
                dev_print!("[ dVFS ]");

                if let UInteger(var_name) = self.stack.pop() {
                    println!("Pointer: {}", self.repository.get_var(var_name as usize));
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
