use crate::vminternals::immediates::Immediates;
use arrayvec::ArrayVec;

const STACK_SIZE: usize = 2000;

pub struct VMStack {
    stack_memory: ArrayVec<Immediates, STACK_SIZE>,
    stack_capacity: usize,
    top: usize,
}

impl VMStack {
    pub fn new() -> VMStack {
        VMStack {
            stack_capacity: STACK_SIZE,
            top: 0,
            stack_memory: ArrayVec::<Immediates, STACK_SIZE>::new(),
        }
    }

    pub fn get_length(&mut self) -> usize {
        self.stack_memory.len()
    }

    pub fn pop(&mut self) -> Immediates {
        if self.top == 0 {
            panic!("[ STACK OVERFLOW ]");
        }

        self.top -= 1;
        self.stack_memory.pop().unwrap()
    }

    pub fn check_empty(&mut self) -> bool {
        if self.stack_memory.is_empty() {
            true
        } else {
            false
        }
    }

    pub fn push(&mut self, data: Immediates) {
        if self.top == self.stack_capacity {
            panic!(
                "[ STACK OVERFLOW ] Stack Capacity: {}, Stack Size: {}",
                self.stack_capacity, self.top
            )
        }

        // if let Immediates::Integer(i) = data {
        //
        //     println!("Data: {}", i);
        //
        // }

        self.stack_memory.push(data);
        self.top += 1;

        // println!("Push: {}", self.top);
    }
}
