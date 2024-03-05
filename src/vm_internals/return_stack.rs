use crate::vm_internals::heap;
use arrayvec::ArrayVec;

/// Fixed value of the return stack size.
const STACK_SIZE: usize = 500;

#[derive(Debug)]
enum ReturnType {
    WithoutPointer(usize),
    WithPointer(heap::Pointer),
}

debug_derive!(
    /// Return Stack implementation.
    pub struct ReturnStack {
        /// Contains all the values pushed into the stack.
        ///
        /// It's an ArrayVec so all the data is contained in the stack.
        /// Being that it has the speed of a real stack.
        return_addresses: ArrayVec<ReturnType, STACK_SIZE>,

        /// The stack capacity.
        stack_capacity: usize,

        /// Points to the latest value pushed into the stack.
        ///
        /// Used mostly for monitoring the stack current size.
        top: usize,
    }
);

impl ReturnStack {
    /// Instantiates the VMStack object and returns it.
    pub fn new() -> ReturnStack {
        ReturnStack {
            stack_capacity: STACK_SIZE,
            top: 0,
            return_addresses: ArrayVec::<ReturnType, STACK_SIZE>::new(),
        }
    }

    /// Used for getting the size of the stack.
    pub fn get_length(&mut self) -> usize {
        self.return_addresses.len()
    }

    /// Used for popping a value from the stack and returning it.
    pub fn pop(&mut self) -> Result<ReturnType, String> {
        if self.top == 0 {
            return Err("[ RETURN STACK UNDERFLOW ]".to_string());
        }

        self.top -= 1;
        Ok(self
            .return_addresses
            .pop()
            .expect("Stack should not be empty"))
    }

    /// Used to check if the stack is empty at the moment.
    pub fn check_empty(&mut self) -> bool {
        self.return_addresses.is_empty()
    }

    /// Used for pushing values into the stack.
    pub fn push(&mut self, data: ReturnType) -> Result<(), String> {
        if self.top == self.stack_capacity {
            return Err(format!(
                "[ RETURN STACK OVERFLOW ] Stack Capacity: {}, Stack Size: {}",
                self.stack_capacity, self.top
            ));
        }

        self.return_addresses.push(data);
        self.top += 1;

        Ok(())
        // println!("{:?}", self.stack_memory);
    }
}
