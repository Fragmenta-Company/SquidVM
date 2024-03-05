use crate::instructiondefs::*;
use crate::vm_internals::immediates::Immediates::{
    self, Array, Binary, Boolean, Float, Integer, MutStr as TypeString, Null, UInteger,
};
use crate::vm_internals::open_window;
use crate::vm_internals::{VMHeap, VMRepository, VMStack};

#[cfg(feature = "green-threads")]
use async_std::task;
#[cfg(feature = "green-threads")]
use async_std::task::JoinHandle;
use std::sync::{Arc, RwLock};

debug_derive!(
    /// Creates new threads that the VM can handle,
    /// almost the same implementation of the VMStarter struct
    pub struct VMThread<'a> {
        pub running: bool,
        pub pc: usize,
        pub instruction: u8,
        pub instructions: Vec<u8>,
        pub data: Immediates,
        pub data_vault: Vec<Immediates>,
        pub stack: VMStack,
        /// Heap is borrowed
        // pub heap: &'a Arc<RwLock<VMHeap>>,
        /// Repository is borrowed
        pub repository: &'a Arc<RwLock<VMRepository>>,
        #[cfg(feature = "green-threads")]
        pub task_handlers: Vec<JoinHandle<Result<(), String>>>,
    }
);

impl VMThread<'_> {
    pub fn new(
        instructions: Vec<u8>,
        data_vault: Vec<Immediates>,
        // heap: &'a Arc<RwLock<VMHeap>>,
        repo: &Arc<RwLock<VMRepository>>,
        stack_size: usize,
    ) -> VMThread {
        VMThread {
            running: true,
            pc: 0,
            instruction: 0,
            instructions,
            data: Null,
            data_vault,
            stack: VMStack::new(stack_size),
            // heap,
            repository: repo,
            #[cfg(feature = "green-threads")]
            task_handlers: Vec::new(),
        }
    }

    /// Contains all the instructions and their implementations.
    /// Receives an instruction and works around it.
    ///
    /// Error out if instruction is _unknown_.
    pub fn instructor(&mut self, instruction: u8) -> Result<(), String> {
        match instruction {
            HALT => {
                dev_print!("[ HALT ]");
                self.running = false;
                Ok(())
            }
            I_ADD => {
                dev_print!("[ iADD ]");
                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    dev_print!("{} {}", v1a, v2a);

                    dev_print!("{}", v1a + v2a);
                    match self.stack.push(Integer(v1a + v2a)) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    };
                    Ok(())
                } else {
                    Err("[ NO INTEGERS ]".to_string())
                }
            }
            I_SUB => {
                dev_print!("[ iSUB ]");
                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a - v2a);
                    match self.stack.push(Integer(v1a - v2a)) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    };
                    Ok(())
                } else {
                    Err("[ NO INTEGERS ]".to_string())
                }
            }
            I_MUL => {
                dev_print!("[ iMUL ]");
                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a * v2a);
                    match self.stack.push(Integer(v1a * v2a)) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    };
                    Ok(())
                } else {
                    Err("[ NO INTEGERS ]".to_string())
                }
            }
            I_DVD => {
                dev_print!("[ iDVD ]");
                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    if (v1a / v2a) as f64 == (v1a as f64 / v2a as f64) {
                        match self.stack.push(Integer(v1a / v2a)) {
                            Err(err) => {
                                return Err(err);
                            }
                            _ => {}
                        };
                        Ok(())
                    } else {
                        match self.stack.push(Float(v1a as f64 / v2a as f64)) {
                            Err(err) => {
                                return Err(err);
                            }
                            _ => {}
                        };
                        Ok(())
                    }
                } else {
                    Err("[ NO INTEGERS ]".to_string())
                }
            }
            F_I_DVD => {
                dev_print!("[ FiDVD ]");
                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    match self.stack.push(Integer(v1a / v2a)) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    };
                    Ok(())
                } else {
                    Err("[ NO INTEGERS ]".to_string())
                }
            }
            F_ADD => {
                dev_print!("[ fADD ]");

                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a + v2a);

                    match self.stack.push(Float(v1a + v2a)) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    };
                    Ok(())
                } else {
                    Err("[ NO FLOATS ]".to_string())
                }
            }
            F_SUB => {
                dev_print!("[ fSUB ]");

                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a - v2a);

                    match self.stack.push(Float(v1a - v2a)) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    };
                    Ok(())
                } else {
                    Err("[ NO FLOATS ]".to_string())
                }
            }
            F_MUL => {
                dev_print!("[ fMUL ]");

                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a * v2a);

                    match self.stack.push(Float(v1a * v2a)) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    };
                    Ok(())
                } else {
                    Err("[ NO FLOATS ]".to_string())
                }
            }
            F_DVD => {
                dev_print!("[ fDVD ]");

                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a / v2a);

                    match self.stack.push(Float(v1a / v2a)) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    };
                    Ok(())
                } else {
                    Err("[ NO FLOATS ]".to_string())
                }
            }
            PDTS => {
                dev_print!("[ PDTS ]");

                let pdts = &self.data;

                match self.stack.push(pdts.clone()) {
                    Err(err) => {
                        return Err(err);
                    }
                    _ => {}
                };
                Ok(())
            }
            PDFS => {
                dev_print!("[ PDFS ]");

                self.data = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                Ok(())
            }
            JMPFD => {
                dev_print!("[ JMPFD ]");

                if let UInteger(i) = self.data {
                    self.pc = i as usize;
                    Ok(())
                } else {
                    Err("[ WRONG ADDRESS ]".to_string())
                }
            }
            JMPFS => {
                dev_print!("[ JMPFS ]");

                let value = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let UInteger(i) = value {
                    self.pc = i as usize;
                    Ok(())
                } else {
                    Err("[ WRONG ADDRESS ]".to_string())
                }
            }
            PRTFS => {
                dev_print!("[ PRTFS ]");

                let value = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                print!("{}", value);

                Ok(())
            }
            PRTFD => {
                dev_print!("[ PRTFD ]");

                print!("{}", &self.data);

                Ok(())
            }
            I_EXP => {
                dev_print!("[ iExp ]");

                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (UInteger(v2), Integer(v1)) = (v2, v1) {
                    match self.stack.push(Integer(v1.pow(v2 as u32))) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    };
                    Ok(())
                } else {
                    Err("[ NO INTEGERS ]".to_string())
                }
            }
            F_EXP => {
                dev_print!("[ fExp ]");

                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (Float(v2), Float(v1)) = (v2, v1) {
                    match self.stack.push(Float(v1.powf(v2))) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    };
                    Ok(())
                } else {
                    Err("[ NO FLOATS ]".to_string())
                }
            }
            F_I_EXP => {
                dev_print!("[ fiExp ]");

                let v2 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let v1 = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let (Integer(v2), Float(v1)) = (v2, v1) {
                    match self.stack.push(Float(v1.powi(v2 as i32))) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    };
                    Ok(())
                } else {
                    Err("[ NO FLOATS ]".to_string())
                }
            }
            AVP => {
                dev_print!("[ AVP ]");

                let var_pointer = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };
                let var_name = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                let repo = Arc::clone(&self.repository);

                if let UInteger(var_name) = var_name {
                    if let UInteger(var_pointer) = var_pointer {
                        repo.write()
                            .unwrap()
                            .add_var(var_name as usize, var_pointer as usize);
                        Ok(())
                    } else {
                        Err("[ INVALID VAR POINTER ]".to_string())
                    }
                } else {
                    Err("[ WRONG VARIABLE NAME ]".to_string())
                }
            }
            D_VFD => {
                dev_print!("[ dVFD ]");

                let repo = Arc::clone(&self.repository);

                if let UInteger(var_name) = self.data {
                    println!(
                        "Pointer: {}",
                        repo.write().unwrap().get_var(var_name as usize)
                    );
                    Ok(())
                } else {
                    Err("[ WRONG VARIABLE NAME ]".to_string())
                }
            }
            D_VFS => {
                dev_print!("[ dVFS ]");

                let repo = Arc::clone(&self.repository);

                let value = match self.stack.pop() {
                    Ok(value) => value,
                    Err(err) => {
                        return Err(err);
                    }
                };

                if let UInteger(var_name) = value {
                    println!(
                        "Pointer: {}",
                        repo.write().unwrap().get_var(var_name as usize)
                    );
                    Ok(())
                } else {
                    Err("[ WRONG VARIABLE NAME ]".to_string())
                }
            }
            0x18 => {
                dev_print!("[ NTW ]");

                #[cfg(feature = "green-threads")]
                unsafe {
                    match task::block_on(open_window()) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err),
                    }
                }

                #[cfg(not(feature = "green-threads"))]
                Err("Green-threads not activated!".to_string())
            }
            #[cfg(feature = "green-threads")]
            // NTASK => {
            //     // ***WIP***
            //     async fn create_new_task(
            //         heap: Arc<RwLock<VMHeap>>,
            //         repo: Arc<RwLock<VMRepository>>,
            //         threadnum: usize,
            //     ) -> Result<(), String> {
            //         let mut thread = VMThread {
            //             running: true,
            //             pc: 0,
            //             instruction: 0,
            //             instructions: vec![
            //                 0x0A, 0x0A, 0x01, 0x14, 0x0A, 0x0A, 0x01, 0x14, 0x0A, 0x0A, 0x01, 0x14,
            //                 0x0A, 0x0A, 0x01, 0x14, 0,
            //             ],
            //             data: Null,
            //             data_vault: vec![
            //                 Integer(1),
            //                 Integer(1),
            //                 Null,
            //                 Null,
            //                 Integer(1),
            //                 Integer(1),
            //                 Null,
            //                 Null,
            //                 Integer(1),
            //                 Integer(1),
            //                 Null,
            //                 Null,
            //                 Integer(1),
            //                 Integer(1),
            //                 Null,
            //                 Null,
            //                 Null,
            //             ],
            //             stack: VMStack::new(),
            //             heap: &heap,
            //             repository: &repo,
            //             task_handlers: Vec::new(),
            //         };
            //
            //         let mut error: Option<String> = None;
            //
            //         while thread.running {
            //             dev_print!("Thread Instructions: {:X?}", thread.instructions);
            //
            //             while thread.pc < thread.instructions.len() {
            //                 let instruction = thread.instructions[thread.pc];
            //                 thread.data = thread.data_vault[thread.pc].clone();
            //                 thread.instruction = instruction;
            //                 thread.pc += 1;
            //                 match thread.instructor(instruction) {
            //                     Ok(_) => {
            //                         #[cfg(feature = "devkit")]
            //                         dev_print!("Thread {} is working!", threadnum);
            //                     }
            //                     Err(err) => {
            //                         error = Some(err);
            //                         thread.running = false;
            //                     }
            //                 };
            //                 dev_print!("{}", thread.pc);
            //                 // println!("Length: {}", self.heap.heap_memory.len());
            //             }
            //
            //             if thread.pc > thread.instructions.len() {
            //                 panic!("[ PROGRAM COUNTER OUT OF RANGE ]");
            //             }
            //
            //             thread.pc += 1;
            //         }
            //
            //         if let Some(err) = error {
            //             Err(err)
            //         } else {
            //             Ok(())
            //         }
            //     }
            //
            //     let heap = Arc::clone(&self.heap);
            //     let repo = Arc::clone(&self.repository);
            //
            //     #[cfg(feature = "devkit")]
            //     let threadnum = self.task_handlers.len();
            //
            //     #[cfg(not(feature = "devkit"))]
            //     let threadnum = 0usize;
            //
            //     let handle = task::spawn(create_new_task(heap, repo, threadnum));
            //
            //     // println!("Data: {:?}", self.data);
            //     if let Boolean(bool) = self.data {
            //         if bool {
            //             self.task_handlers.push(handle);
            //         }
            //     }
            //     Ok(())
            // }
            NTHRD => Err("Threads cannot be created inside other tasks/threads!".to_string()),
            _ => Err("[ UNKNOWN INSTRUCTION ]".to_string()),
        }
    }
}
