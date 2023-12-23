use crate::instructiondefs::*;
use crate::sqdbin_reader::FileReader;
use crate::vm_internals::immediates::Immediates::{
    self, Array, Binary, Boolean, Float, Integer, Null, RefPtr, String as TypeString, UInteger,
};
use crate::vm_internals::vm_threads::VMThread;
use crate::vm_internals::{VMHeap, VMRepository, VMStack};
use async_std::task;
use async_std::task::JoinHandle;
use std::process;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};


/// Handles errors while pop from the stack
fn handle_stack_err(result: Result<Immediates, String>) -> Immediates {
    match result {
        Ok(obj) => obj,
        Err(err) => {
            eprintln!("\x1B[41m{err}\x1B[0m");
            process::exit(11);
        }
    }
}

/// Handles error while pushing to the stack
fn handle_stack_push_err(result: Result<(), String>) {
    match result {
        Err(err) => {
            eprintln!("\x1B[41m{err}\x1B[0m");
            process::exit(12);
        }
        _ => {}
    }
}

/// Print any type, even pointers
pub fn print_any(printable: Immediates) {
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

/// Open new window ***WIP***
pub async fn open_window() -> Result<(), String> {

    let sdl_ctx = sdl2::init().unwrap();

    let video_sub = sdl_ctx.video()?;

    let mut window = video_sub.window("Random line drawer", 800, 600)
        .position_centered().vulkan().resizable().build().map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_ctx.event_pump()?;

    loop {

        let mut rng = rand::thread_rng();

        let first:i32 = rng.gen_range(0..=2000);
        let second:i32 = rng.gen_range(0..=2000);
        let third:i32 = rng.gen_range(0..=2000);
        let forth:i32 = rng.gen_range(0..=2000);

        for event in event_pump.poll_iter() {
            match event {
                // Quit the program if the window is closed or the escape key is pressed
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Ok(()),
                _ => {}
            }
        }

        // Clear the canvas with black color
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        // Set the draw color to red
        canvas.set_draw_color(Color::RGB(255, 0, 0));

        // Draw something on the canvas here...
        canvas.draw_line(Point::new(first ,second), Point::new(third, forth))?;
        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.draw_line(Point::new(forth ,first), Point::new(second, third))?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_line(Point::new(third ,first), Point::new(first, forth))?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_line(Point::new(third ,forth), Point::new(second, forth))?;

        canvas.draw_rect(Rect::new(forth, forth, third as u32, first as u32))?;
        canvas.draw_rect(Rect::new(first, forth, first as u32, second as u32))?;
        canvas.draw_rect(Rect::new(third, forth, third as u32, forth as u32))?;
        canvas.draw_rect(Rect::new(first, forth, third as u32, second as u32))?;

        // Present the canvas on the window
        canvas.present();

        // Sleep for 1/60th of a second to limit the frame rate
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

}

/// Put #[derive(Debug)] into struct if devkit feature is enabled
#[cfg(feature = "devkit")]
macro_rules! debug_derive {
    ($($item:tt)*) => {
        #[derive(Debug)]
        $($item)*
    };
}

/// Put #[derive(Debug)] into struct if devkit feature is enabled
#[cfg(not(feature = "devkit"))]
macro_rules! debug_derive {
    ($($item:tt)*) => {
        $($item)*
    };
}

debug_derive!(
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
        heap: Arc<RwLock<VMHeap>>,

        /// It's used to store pointer for heap values, so it can be used as a global variable storage.
        ///
        /// <p style="color: #FF6E6E;">Warning:</p>
        ///
        /// * All objects used by the repository will **ignored** by the garbage collcetor.
        /// * Only use global variables when they are needed, since they can be a security risk.
        repository: Arc<RwLock<VMRepository>>,

        pub handlers: Vec<JoinHandle<Result<(), String>>>,
    }
);

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
            heap: Arc::new(RwLock::from(VMHeap::new(heap_size))),
            repository: Arc::new(RwLock::from(VMRepository::new(repository_size))),
            handlers: Vec::new(),
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

        dev_print!("Instructions: {:X?}", self.instructions);

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
                let v2 = handle_stack_err(self.stack.pop());
                let v1 = handle_stack_err(self.stack.pop());

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    dev_print!("{} {}", v1a, v2a);

                    dev_print!("{}", v1a + v2a);
                    handle_stack_push_err(self.stack.push(Integer(v1a + v2a)));
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            I_SUB => {
                dev_print!("[ iSUB ]");
                let v2 = handle_stack_err(self.stack.pop());
                let v1 = handle_stack_err(self.stack.pop());

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a - v2a);
                    handle_stack_push_err(self.stack.push(Integer(v1a - v2a)));
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            I_MUL => {
                dev_print!("[ iMUL ]");
                let v2 = handle_stack_err(self.stack.pop());
                let v1 = handle_stack_err(self.stack.pop());

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a * v2a);
                    handle_stack_push_err(self.stack.push(Integer(v1a * v2a)));
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            I_DVD => {
                dev_print!("[ iDVD ]");
                let v2 = handle_stack_err(self.stack.pop());
                let v1 = handle_stack_err(self.stack.pop());

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    if (v1a / v2a) as f64 == (v1a as f64 / v2a as f64) {
                        handle_stack_push_err(self.stack.push(Integer(v1a / v2a)));
                    } else {
                        handle_stack_push_err(self.stack.push(Float(v1a as f64 / v2a as f64)));
                    }
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            F_I_DVD => {
                dev_print!("[ FiDVD ]");
                let v2 = handle_stack_err(self.stack.pop());
                let v1 = handle_stack_err(self.stack.pop());

                if let (Integer(v1a), Integer(v2a)) = (v1, v2) {
                    handle_stack_push_err(self.stack.push(Integer(v1a / v2a)));
                } else {
                    panic!("[ NO INTEGERS ]");
                }
            }
            F_ADD => {
                dev_print!("[ fADD ]");

                let v2 = handle_stack_err(self.stack.pop());
                let v1 = handle_stack_err(self.stack.pop());

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a + v2a);

                    handle_stack_push_err(self.stack.push(Float(v1a + v2a)));
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            F_SUB => {
                dev_print!("[ fSUB ]");

                let v2 = handle_stack_err(self.stack.pop());
                let v1 = handle_stack_err(self.stack.pop());

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a - v2a);

                    handle_stack_push_err(self.stack.push(Float(v1a - v2a)));
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            F_MUL => {
                dev_print!("[ fMUL ]");

                let v2 = handle_stack_err(self.stack.pop());
                let v1 = handle_stack_err(self.stack.pop());

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a * v2a);

                    handle_stack_push_err(self.stack.push(Float(v1a * v2a)));
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            F_DVD => {
                dev_print!("[ fDVD ]");

                let v2 = handle_stack_err(self.stack.pop());
                let v1 = handle_stack_err(self.stack.pop());

                if let (Float(v1a), Float(v2a)) = (v1, v2) {
                    dev_print!("{}", v1a / v2a);

                    handle_stack_push_err(self.stack.push(Float(v1a / v2a)));
                } else {
                    panic!("[ NO FLOATS ]");
                }
            }
            PDTS => {
                dev_print!("[ PDTS ]");

                let pdts = &self.data;

                handle_stack_push_err(self.stack.push(pdts.clone()));
            }
            PDFS => {
                dev_print!("[ PDFS ]");

                self.data = handle_stack_err(self.stack.pop());
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

                if let UInteger(i) = handle_stack_err(self.stack.pop()) {
                    self.pc = i as usize;
                } else {
                    panic!("[ WRONG ADDRESS ]");
                }
            }
            PRTFS => {
                dev_print!("[ PRTFS ]");

                if let TypeString(s) = handle_stack_err(self.stack.pop()) {
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

                if let (UInteger(v2), Integer(v1)) = (
                    handle_stack_err(self.stack.pop()),
                    handle_stack_err(self.stack.pop()),
                ) {
                    handle_stack_push_err(self.stack.push(Integer(v1.pow(v2 as u32))));
                } else {
                    panic!("[ NO INTEGERS ]")
                }
            }
            F_EXP => {
                dev_print!("[ fExp ]");

                if let (Float(v2), Float(v1)) = (
                    handle_stack_err(self.stack.pop()),
                    handle_stack_err(self.stack.pop()),
                ) {
                    handle_stack_push_err(self.stack.push(Float(v1.powf(v2))));
                } else {
                    panic!("[ NO FLOATS ]")
                }
            }
            F_I_EXP => {
                dev_print!("[ fiExp ]");

                if let (Integer(v2), Float(v1)) = (
                    handle_stack_err(self.stack.pop()),
                    handle_stack_err(self.stack.pop()),
                ) {
                    handle_stack_push_err(self.stack.push(Float(v1.powi(v2 as i32))));
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

                print_any(handle_stack_err(self.stack.pop()));
            }
            AVP => {
                dev_print!("[ AVP ]");

                let var_pointer = handle_stack_err(self.stack.pop());
                let var_name = handle_stack_err(self.stack.pop());

                let repo = Arc::clone(&self.repository);

                if let UInteger(var_name) = var_name {
                    if let UInteger(var_pointer) = var_pointer {
                        repo.write()
                            .unwrap()
                            .add_var(var_name as usize, var_pointer as usize);
                    } else {
                        panic!("[ INVALID VAR POINTER ]");
                    }
                } else {
                    panic!("[ INVALID VAR NAME ]");
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
                } else {
                    panic!("[ WRONG VARIABLE NAME ]");
                }
            }
            D_VFS => {
                dev_print!("[ dVFS ]");

                let repo = Arc::clone(&self.repository);

                if let UInteger(var_name) = handle_stack_err(self.stack.pop()) {
                    println!(
                        "Pointer: {}",
                        repo.write().unwrap().get_var(var_name as usize)
                    );
                } else {
                    panic!("[ WRONG VARIABLE NAME ]");
                }
            }
            0x18 => {
                dev_print!("[ NTW ]");

                task::block_on(open_window());
            }
            0x19 => {
                async fn create_new_thread(
                    heap: Arc<RwLock<VMHeap>>,
                    repo: Arc<RwLock<VMRepository>>,
                    threadnum: usize,
                ) -> Result<(), String> {
                    let mut thread = VMThread {
                        running: true,
                        pc: 0,
                        instruction: 0,
                        instructions: vec![
                            0x0A, 0x0A, 0x01, 0x14, 0x0A, 0x0A, 0x01, 0x14, 0x0A, 0x0A, 0x01, 0x14,
                            0x0A, 0x0A, 0x01, 0x14, 0x01, 0,
                        ],
                        data: Null,
                        data_vault: vec![
                            Integer(1),
                            Integer(1),
                            Null,
                            Null,
                            Integer(1),
                            Integer(1),
                            Null,
                            Null,
                            Integer(1),
                            Integer(1),
                            Null,
                            Null,
                            Integer(1),
                            Integer(1),
                            Null,
                            Null,
                            Null,
                            Null,
                        ],
                        stack: VMStack::new(),
                        heap: &heap,
                        repository: &repo,
                    };

                    let mut error: Option<String> = None;

                    while thread.running {
                        dev_print!("Thread Instructions: {:X?}", thread.instructions);

                        while thread.pc < thread.instructions.len() {
                            let instruction = thread.instructions[thread.pc];
                            thread.data = thread.data_vault[thread.pc].clone();
                            thread.instruction = instruction;
                            thread.pc += 1;
                            match thread.instructor(instruction) {
                                Ok(_) => {
                                    #[cfg(feature = "devkit")]
                                    dev_print!("Thread {} is working!", threadnum);
                                }
                                Err(err) => {
                                    error = Some(err);
                                    thread.running = false;
                                }
                            };
                            dev_print!("{}", thread.pc);
                            // println!("Length: {}", self.heap.heap_memory.len());
                        }

                        if thread.pc > thread.instructions.len() {
                            panic!("[ PROGRAM COUNTER OUT OF RANGE ]");
                        }

                        thread.pc += 1;
                    }

                    if let Some(err) = error {
                        Err(err)
                    } else {
                        Ok(())
                    }
                }

                let heap = Arc::clone(&self.heap);
                let repo = Arc::clone(&self.repository);

                #[cfg(feature = "devkit")]
                let threadnum = self.handlers.len();

                #[cfg(not(feature = "devkit"))]
                let threadnum = 0usize;

                let handle = task::spawn(create_new_thread(heap, repo, threadnum));

                self.handlers.push(handle);
            }
            _ => {
                panic!("[ UNKNOWN INSTRUCTION ]")
            }
        }
    }
}
