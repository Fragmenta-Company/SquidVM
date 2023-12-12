use crate::vminternals::immediates::Immediates;
use arrayvec::ArrayVec;

/// Fixed value of the stack size.
const STACK_SIZE: usize = 2000;

/// Stack implementation.
pub struct VMStack {
    /// Contains all the values pushed into the stack.
    ///
    /// It's an ArrayVec so all the data is contained in the stack.
    /// Being that it has the speed of a real stack.
    stack_memory: ArrayVec<Immediates, STACK_SIZE>,

    /// The stack capacity.
    stack_capacity: usize,

    /// Points to the latest value pushed into the stack.
    ///
    /// Used mostly for monitoring the stack current size.
    top: usize,
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
    pub fn pop(&mut self) -> Immediates {
        if self.top == 0 {
            panic!("[ STACK UNDERFLOW ]");
        }

        self.top -= 1;
        self.stack_memory.pop().expect("Stack should not be empty")
    }

    /// Used to check if the stack is empty at the moment.
    pub fn check_empty(&mut self) -> bool {
        self.stack_memory.is_empty()
    }

    /// Used for pushing values into the stack.
    pub fn push(&mut self, data: Immediates) {
        if self.top == self.stack_capacity {
            panic!(
                "[ STACK OVERFLOW ] Stack Capacity: {}, Stack Size: {}",
                self.stack_capacity, self.top
            )
        }

        self.stack_memory.push(data);
        self.top += 1;

        // println!("{:?}", self.stack_memory);
    }
}
