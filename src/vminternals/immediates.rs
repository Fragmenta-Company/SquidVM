use std::mem;

/// ## Immediates are the objects types containing the value inside them
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Immediates {
    /// Null type
    Null,
    /// Boolean type
    Boolean(bool),
    /// Integer (i64) type
    Integer(i64),
    /// UInteger (u64) type
    UInteger(u64),
    /// Float (f64) type
    Float(f64),
    /// String type
    String(String),
    /// Binary (`Vec<u8>`) type
    Binary(Vec<u8>),
    /// Array of Immediates type
    Array(Vec<Immediates>),
}

/// ## ImmediatesType are the objects types NOT containing the value inside them, just the type.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum ImmediatesType {
    /// Null type
    Null,
    /// Boolean type
    Boolean,
    /// Integer (i64) type
    Integer,
    /// UInteger (u64) type
    UInteger,
    /// Float (f64) type
    Float,
    /// String type
    String,
    /// Binary (`Vec<u8>`) type
    Binary,
    /// Array of Immediates type
    Array,
}

/// ## Creates a function to serialize Immediates to sequences of bytes
pub trait Serialize {
    /// Serialize Immediates to Vector of bytes (`Vec<u8>`)
    fn serialize(&self) -> Vec<u8>;
    /// Serialize Immediates to Vector of bytes (`Vec<u8>`) for the heap
    fn serialize_heap(&self) -> Vec<u8>;
}

/// ## Used for turning Immediates to Types, so you can get Types from Immediates values
pub trait ImmediateType {
    /// Immediates to ImmediatesType
    fn to_immediate_type(&self) -> ImmediatesType;
}

impl ImmediateType for Immediates {
    fn to_immediate_type(&self) -> ImmediatesType {
        match self {
            Immediates::Null => ImmediatesType::Null,
            Immediates::Boolean(_) => ImmediatesType::Boolean,
            Immediates::Integer(_) => ImmediatesType::Integer,
            Immediates::UInteger(_) => ImmediatesType::UInteger,
            Immediates::Float(_) => ImmediatesType::Float,
            Immediates::String(_) => ImmediatesType::String,
            Immediates::Binary(_) => ImmediatesType::Binary,
            Immediates::Array(_) => ImmediatesType::Array,
        }
    }
}

/// ## Turn bool to a vector of bytes.
fn bool_to_bytes(value: bool) -> Vec<u8> {
    if value {
        1u8.to_le_bytes().to_vec()
    } else {
        0u8.to_le_bytes().to_vec()
    }
}

impl Serialize for Immediates {
    fn serialize(&self) -> Vec<u8> {
        match self {
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
        }
    }

    fn serialize_heap(&self) -> Vec<u8> {
        match self {
            Immediates::Null => vec![0u8; mem::size_of::<Immediates>()],
            Immediates::Boolean(booval) => bool_to_bytes(*booval),
            Immediates::Integer(i) => {
                let mut bytes = vec![0u8; mem::size_of::<i64>()];
                bytes.copy_from_slice(&i.to_le_bytes());
                bytes
            }
            Immediates::UInteger(ui) => {
                let mut bytes = vec![0u8; mem::size_of::<u64>()];
                bytes.copy_from_slice(&ui.to_le_bytes());
                bytes
            }
            Immediates::Float(f) => {
                let mut bytes = vec![0u8; mem::size_of::<f64>()];
                bytes.copy_from_slice(&f.to_le_bytes());
                bytes
            }
            Immediates::String(string) => string.clone().into_bytes(),
            Immediates::Binary(bin) => bin.clone(),
            Immediates::Array(_) => {
                panic!("Array not permitted for instance");
            }
        }
    }
}
