use crate::errdef::HEAP_ALLOC_ERR;
use crate::vm_internals::immediates::*;
use std::{mem, process};
#[allow(unused_imports)]
use Colors::{Black, Gray, White};

debug_derive!(
    /// Heap implementation
    pub struct VMHeap {
        heap_memory: Vec<AllocatedObject>,
        heap_capacity: usize,
        heap_free: usize,
    }
);

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Allocation {
    pub index: usize,
    pub size: usize,
    pub immediate_type: ImmediatesType,
}

/// ## Contain the colors the AllocatedObject will have assigned.
///
/// Based on the tri-color marking tracing garbage collector implementation.
/// References here:
/// [Tracing Garbage Collector](https://en.wikipedia.org/wiki/Tracing_garbage_collection)
#[derive(Clone, PartialEq, PartialOrd, Debug)]
enum Colors {
    /// **Condemned** objects
    White,

    /// _Soon to be checked_ objects
    Gray,

    /// ***Passed*** objects
    Black,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct AllocatedObject {
    pub data: Vec<u8>,
    marked: Colors,
}

impl VMHeap {
    pub fn new(heap_capacity: usize) -> Self {
        VMHeap {
            heap_memory: Vec::new(),
            heap_capacity,
            heap_free: heap_capacity,
        }
    }

    pub fn malloc(&mut self, data: Immediates) -> Allocation {
        let serialized = data.serialize_heap();

        println!("Size: {}", mem::size_of_val(&serialized));
        println!("Content {:?}", &serialized.len());

        let object = AllocatedObject {
            data: serialized,
            marked: White,
        };

        let size = mem::size_of_val(&object);

        let malloc = Allocation {
            index: self.heap_memory.len(),
            size,
            immediate_type: data.to_immediate_type(),
        };

        if self.heap_free > size {
            self.heap_free -= size;
        } else {
            eprintln!("\x1B[41m[ HEAP OVERFLOW ]\x1b[0m");
            process::exit(HEAP_ALLOC_ERR);
        }

        self.heap_memory.push(object);

        dev_print!("{:?}", self.heap_memory);

        malloc
    }

    pub fn get_obj(&mut self, address: usize) -> AllocatedObject {
        // let obj = self.heap_memory[address].as_ptr();

        self.heap_memory[address].clone()
    }

    pub fn free(&mut self, alloc: Allocation) {
        dev_print!("Alloc Index: {}", &alloc.index);
        dev_print!("Alloc Size: {}", &alloc.size);
        dev_print!("Alloc Type: {:?}", &alloc.immediate_type);

        self.heap_free += alloc.size;

        dev_print!("{:?}", self.heap_memory.swap_remove(alloc.index));

        drop(alloc);
    }

    pub fn mark_roots() {}
}
