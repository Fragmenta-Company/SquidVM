use crate::vminternals::immediates::Immediates;
use std::mem;

pub struct VMHeap {
    heap_memory: Vec<u8>,
    heap_capacity: usize,
}

pub struct Allocation {
    start_address: usize,
    size: usize,
    immediate_type: ImmediatesType,
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum ImmediatesType {
    Null,
    Boolean,
    Integer,
    UInteger,
    Float,
    String,
    Binary,
    Array,
}

fn bool_to_bytes(value: bool) -> Vec<u8> {
    if value {
        1u8.to_le_bytes().to_vec()
    } else {
        0u8.to_le_bytes().to_vec()
    }
}

impl VMHeap {
    pub fn new(heap_capacity: usize) -> Self {
        VMHeap {
            heap_memory: Vec::with_capacity(heap_capacity),
            heap_capacity,
        }
    }

    pub fn malloc(&mut self, data: Immediates) -> Result<Allocation, String> {
        let heap_mem_len = self.heap_memory.len();

        if self.heap_capacity == heap_mem_len {
            return Err(format!(
                "[ HEAP OVERFLOW ]: Heap Length: {} | Heap Capacity: {}",
                heap_mem_len, self.heap_capacity
            ));
        }

        let serialized_bytes = match &data {
            Immediates::Null => vec![0u8; mem::size_of::<Immediates>()],
            Immediates::Boolean(booval) => bool_to_bytes(*booval),
            Immediates::Integer(i) => {
                let mut bytes = vec![0u8; mem::size_of::<Immediates>()];
                bytes.copy_from_slice(&i.to_le_bytes());
                bytes
            }
            Immediates::UInteger(ui) => {
                let mut bytes = vec![0u8; mem::size_of::<Immediates>()];
                bytes.copy_from_slice(&ui.to_le_bytes());
                bytes
            }
            Immediates::Float(f) => {
                let mut bytes = vec![0u8; mem::size_of::<Immediates>()];
                bytes.copy_from_slice(&f.to_le_bytes());
                bytes
            }
            Immediates::String(string) => string.clone().into_bytes(),
            Immediates::Binary(bin) => bin.clone(),
            Immediates::Array(_) => {
                panic!("Array not permited for instance");
            }
        };

        if heap_mem_len + serialized_bytes.len() <= self.heap_capacity {
            let start_address = self.heap_memory.len();

            self.heap_memory.extend_from_slice(&serialized_bytes);

            Ok(Allocation {
                start_address,
                size: serialized_bytes.len(),
                immediate_type: match data {
                    Immediates::Null => ImmediatesType::Null,
                    Immediates::Boolean(_) => ImmediatesType::Boolean,
                    Immediates::Integer(_) => ImmediatesType::Integer,
                    Immediates::UInteger(_) => ImmediatesType::UInteger,
                    Immediates::Float(_) => ImmediatesType::Float,
                    Immediates::String(_) => ImmediatesType::String,
                    Immediates::Binary(_) => ImmediatesType::Binary,
                    Immediates::Array(_) => ImmediatesType::Array,
                },
            })
        } else {
            Err(format!(
                "[ HEAP OVERFLOW ]: Heap Length: {} | Heap Capacity: {} | Object size: {}",
                heap_mem_len,
                self.heap_capacity,
                serialized_bytes.len()
            ))
        }
    }

    pub fn free() {}
}
