use crate::vminternals::immediates::Immediates;
use fnv::FnvHashMap;
pub struct VMHeap {
    pub heap_memory: FnvHashMap<usize, Immediates>,
    pub heap_capacity: usize,
}

impl VMHeap {
    pub fn new(heap_capacity: usize) -> VMHeap {
        VMHeap {
            heap_memory: FnvHashMap::with_capacity_and_hasher(heap_capacity, Default::default()),
            heap_capacity,
        }
    }

    pub fn add_var(&mut self, var_name: usize, var_data: Immediates) {
        if self.heap_capacity == self.heap_memory.len() {
            panic!("[ HEAP OVERFLOW ]")
        }

        self.heap_memory.insert(var_name, var_data);
    }

    pub fn remove_var(&mut self, var_name: usize) {
        if !self.heap_memory.contains_key(&var_name) {
            panic!("[ UNDEFINED VARIABLE ADDRESS ]")
        }

        self.heap_memory.remove(&var_name);
    }

    pub fn get_var(&mut self, var_name: usize) -> &Immediates {
        self.heap_memory.get(&var_name).unwrap()
    }

    pub fn pop_var(&mut self, var_name: usize) -> Immediates {
        if 0 == self.heap_memory.len() {
            panic!("[ HEAP OVERFLOW ]")
        }

        self.heap_memory.remove(&var_name).unwrap()
    }

    pub fn clear_heap(&mut self) {
        self.heap_memory.clear();
    }
}
