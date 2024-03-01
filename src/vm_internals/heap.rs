use crate::errdef::HEAP_ALLOC_ERR;
use crate::vm_internals::immediates::*;
use ahash::AHashMap;
use std::mem::{size_of, size_of_val};
use std::sync::RwLock;
#[allow(unused_imports)]
use Colors::{Black, Gray, White};

#[derive(Debug)]
pub enum Colors {
    White,
    Gray,
    Black,
}

debug_derive!(
    pub struct VMHeap {
        pub heap: Vec<Option<Region>>,     // Heap contains heap regions
        pub index: AHashMap<usize, usize>, // Region index
        pub free: usize,                   // In bytes
        pub capacity: usize,               // In bytes
        pub threads: usize,                // Number of threads active
        pub tasks: usize,                  // Number of tasks active
    }
);

impl VMHeap {
    pub fn new(capacity: usize) -> Self {
        let main = Region::new(false, None, None);

        let mut heap = Vec::new();
        heap.push(Some(main));

        let mut hashmap = AHashMap::new();

        hashmap.insert(0, 0);

        VMHeap {
            heap,
            index: hashmap,
            free: capacity,
            capacity,
            threads: 1,
            tasks: 0,
        }
    }

    pub fn new_empty(capacity: usize) -> Self {
        VMHeap {
            heap: Vec::new(),
            index: AHashMap::new(),
            free: capacity,
            capacity,
            threads: 0,
            tasks: 0,
        }
    }

    pub fn find_first_index(&mut self) -> (usize, usize) {
        let mut key = 0;

        while self.index.contains_key(&key) {
            key += 1;
        }

        let mut index = 0;

        if self.heap.len() != 0 {
            while !self.heap[index].is_none() {
                index += 1;
                if index + 1 > self.heap.len() {
                    index = self.heap.len() + 1;
                    break;
                }
            }
        } else {
            index+=1;
        }

        index -= 1;

        (key, index)
    }

    pub fn allocate_global_region(&mut self) -> usize {
        let region = Region::new(true, None, None);

        let (key, index) = self.find_first_index();

        // println!("{:?}", self.find_first_index());

        self.index.insert(key, index);

        if index >= self.heap.len() {
            self.heap.push(Some(region));
        } else {
            self.heap[index] = Some(region);
        }

        key
    }

    pub fn allocate_thread_region(&mut self) -> usize {
        let region = Region::new(false, Some(self.threads + 1), None);

        let (key, index) = self.find_first_index();

        self.index.insert(key, index);

        if index >= self.heap.len() {
            self.heap.push(Some(region));
        } else {
            self.heap[index] = Some(region);
        }

        self.threads += 1;

        key
    }

    pub fn allocate_task_region(&mut self) -> usize {
        let region = Region::new(false, None, Some(self.tasks + 1));

        let (key, index) = self.find_first_index();

        self.index.insert(key, index);

        if index >= self.heap.len() {
            self.heap.push(Some(region));
        } else {
            self.heap[index] = Some(region);
        }

        self.tasks += 1;

        key
    }
}

debug_derive!(
    pub struct Region {
        pub memory: Vec<Option<RwLock<AllocatedObject>>>,
        pub index: AHashMap<usize, usize>, // Object Index
        pub assigned_thread: Option<usize>,
        pub assigned_task: Option<usize>,
        pub global: bool,
    }
);

impl Region {
    pub fn new(global: bool, thread: Option<usize>, task: Option<usize>) -> Self {
        Region {
            memory: Vec::new(),
            index: AHashMap::new(),
            assigned_thread: thread,
            assigned_task: task,
            global,
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataType {
    Immediate(Immediates),
    Function(Function),
    Composite(Composites),
    Pointer(Pointer),
}

#[derive(Debug, Clone)]
pub enum Composites {
    Vector(Vec<DataType>),
    Sequence(Sequence),
    Struct(Struct),
}

#[derive(Debug, Clone)]
pub struct Sequence {
    data: Vec<DataType>,
    length: usize,
}

#[derive(Debug, Clone)]
pub struct Pointer {
    point_to: usize,
    is_fn: bool,
    thread: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub(crate) instructions: Vec<u8>,
    pub(crate) data: Vec<Immediates>,
    pub(crate) asynchronous: bool,
}

#[derive(Debug, Clone)]
pub struct Struct {
    attributes: AHashMap<usize, Immediates>,
    methods: AHashMap<usize, (Option<Function>, Option<Pointer>)>,
}

#[derive(Debug, Clone)]
pub struct AllocatedObject {
    pub data: DataType,
    pub size: usize,
}

impl AllocatedObject {
    pub fn new(data: DataType) -> Self {
        let mut size = size_of_val(&data);

        if let DataType::Immediate(immediate) = &data {
            if let Immediates::Integer(_) = immediate {
                size += size_of::<i64>();
            }

            if let Immediates::Float(_) = immediate {
                size += size_of::<f64>();
            }

            if let Immediates::Boolean(_) = immediate {
                size += size_of::<bool>();
            }

            if let Immediates::UInteger(_) = immediate {
                size += size_of::<u64>();
            }

            if let Immediates::MutStr(string) = immediate {
                let string_size = string.len();

                size -= 24;

                size += string_size;
            };
        };

        if let DataType::Function(func) = &data {
            let idk2 = size_of_val(&func.data);

            let idk = size_of_val(&func.instructions);

            println!("{} {}", idk2, idk);
        };

        AllocatedObject { size, data }
    }
}
