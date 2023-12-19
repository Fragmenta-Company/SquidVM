use std::mem;
use crate::vminternals::immediates::*;
use std::sync::atomic::{AtomicPtr, Ordering};
use Colors::{Black, Gray, White};

/// Heap implementation
#[cfg(feature = "devkit")]
#[derive(Debug)]
pub struct VMHeap {
    heap_memory: Vec<AtomicPtr<AllocatedObject>>,
    heap_capacity: usize,
}

#[cfg(not(feature = "devkit"))]
pub struct VMHeap {
    heap_memory: Vec<AtomicPtr<AllocatedObject>>,
    heap_capacity: usize,
}

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
            heap_memory: Vec::with_capacity(heap_capacity),
            heap_capacity,
        }
    }

    pub fn malloc(&mut self, data: Immediates) -> Allocation {

        let serialized = data.serialize_heap();

        println!("Size: {}", mem::size_of_val(&serialized));
        println!("Content {:?}", &serialized.len());

        let object = Box::new(AllocatedObject {
            data: serialized,
            marked: White,
        });

        let pointer = Box::into_raw(object);

        let size = mem::size_of_val( unsafe { &*pointer });

        let malloc = Allocation {
            index: self.heap_memory.len(),
            size,
            immediate_type: data.to_immediate_type(),
        };

        self.heap_memory.push(AtomicPtr::new(pointer));

        dev_print!("{:?}", self.heap_memory);

        malloc

    }

    pub fn get_obj(&mut self, address: usize) -> AllocatedObject {


        // let obj = self.heap_memory[address].as_ptr();

        unsafe { self.heap_memory[address].load(Ordering::Relaxed).read()}

    }

    pub fn free(&mut self, alloc: Allocation) {

        dev_print!("Alloc Index: {}", &alloc.index);
        dev_print!("Alloc Size: {}", &alloc.size);
        dev_print!("Alloc Type: {:?}", &alloc.immediate_type);

        self.heap_memory.swap_remove(alloc.index);

        drop(alloc);

    }


}
