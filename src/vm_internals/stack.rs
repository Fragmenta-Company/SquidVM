use std::fmt::{Debug, Formatter};
use crate::vm_internals::immediates::Immediates;
use arrayvec::ArrayVec;

/// Fixed value of the stack size.
const STACK_SIZE: usize = 2000;


/// Stack implementation.
pub struct VMStack {
    /// Contains all the values pushed into the stack.
    ///
    /// It's an ArrayVec so all the data is contained in the stack.
    /// Being that it has the speed of a real stack.
    pub stack_memory: ArrayVec<Immediates, STACK_SIZE>,

    /// The stack capacity.
    pub stack_capacity: usize,

    /// Points to the latest value pushed into the stack.
    ///
    /// Used mostly for monitoring the stack current size.
    pub(crate) top: usize, 
}

impl Debug for VMStack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stack => [\n\tContents => {:?},\n\tCapacity => {},\n\tTop => {},\n]", self.stack_memory, self.stack_capacity, self.top)
    }
}

impl VMStack {
    /// Instantiates the VMStack object and returns it.
    pub fn new() -> VMStack {
        VMStack {
            stack_capacity: STACK_SIZE,
            top: 0,
            stack_memory: ArrayVec::<Immediates, STACK_SIZE>::new(),
        }
    }

    /// Used for getting the size of the stack.
    pub fn get_length(&mut self) -> usize {
        self.stack_memory.len()
    }

    /// Used for popping a value from the stack and returning it.
    pub fn pop(&mut self) -> Result<Immediates, String> {
        if self.top == 0 {
            return Err("[ STACK UNDERFLOW ]".to_string());
        }

        self.top -= 1;
        Ok(self.stack_memory.pop().expect("Stack should not be empty"))
    }

    /// Used to check if the stack is empty at the moment.
    pub fn check_empty(&mut self) -> bool {
        self.stack_memory.is_empty()
    }

    /// Used for pushing values into the stack.
    pub fn push(&mut self, data: Immediates) -> Result<(), String> {
        if self.top == self.stack_capacity {
            return Err(format!(
                "[ STACK OVERFLOW ] Stack Capacity: {}, Stack Size: {}",
                self.stack_capacity, self.top
            ));
        }

        self.stack_memory.push(data);
        self.top += 1;

        Ok(())
        // println!("{:?}", self.stack_memory);
    }
}
