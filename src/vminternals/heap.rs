use crate::vminternals::immediates::*;
use std::sync::atomic::{AtomicPtr, Ordering};
use Colors::{Black, Gray, White};

/// Heap implementation
pub struct VMHeap {
    heap_memory: Vec<AtomicPtr<AllocatedObject>>,
    heap_capacity: usize,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Allocation {
    start_address: usize,
    size: usize,
    immediate_type: ImmediatesType,
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

    // pub fn malloc(&mut self, value: Immediates){
    //
    //     let mut obj = Box::new(AllocatedObject {
    //         data: vec![0u8, 1u8],
    //         marked: Black
    //     });
    //
    //     let obj_pointer: *mut AllocatedObject = Box::into_raw(obj);
    //
    //     self.heap_memory.push(AtomicPtr::new(obj_pointer));
    //
    // }
    //
    // pub fn get_obj(&mut self) -> *mut AllocatedObject {
    //
    //     let mut obj = self.heap_memory.pop().unwrap();
    //
    //     let mut another_obj = obj.load(Ordering::Relaxed);
    //
    //     let allocated_object = unsafe { &mut *another_obj };
    //
    //     println!("{:?}", allocated_object.data);
    //
    //     allocated_object
    //
    //
    // }

}
