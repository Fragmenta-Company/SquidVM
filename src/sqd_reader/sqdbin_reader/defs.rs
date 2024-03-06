/// .sqd file extension. (".sqdbin" was renamed to ".sqd")
pub const SQDBIN: &str = ".sqd";

/// Size of the file's header that stores the metadata.
pub const HEADER_SIZE: u64 = 32;

/// Size of the compiler name in the metadata.
pub const COMPILER_NAME_SIZE: u32 = 22;

/// Size of the metadata identifier.
pub const METADATA_IDENTIFIER_BYTE: u64 = 1;

/// Size of the major number (u32)
pub const MAJOR_SIZE: u64 = 4;

/// Size of the minor and patch number (u16)
pub const MINOR_PATCH_SIZE: u64 = 2;

/// Major version of the SquidVM instance.
pub const VM_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");

/// Minor version of the SquidVM instance.
pub const VM_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");

// Primitives

/// Defines the null type for reading from a file.
pub const NULL: u8 = 0x00;

/// Defines the bool type for reading from a file.
pub const BOOL: u8 = 0x01;

/// Defines the integer (i64) type for reading from a file.
pub const INTEGER: u8 = 0x02;

/// Defines the unsigned integer (u64) type for reading from a file.
pub const UINTEGER: u8 = 0x03;

/// Defines the float (f64) type for reading from a file.
pub const FLOAT: u8 = 0x04;

/// Defines the TinyFloat (f32) type for reading from a file.
pub const TINYFLOAT: u8 = 0x05;

/// Defines the byte (u8) type for reading from a file;
pub const BYTE: u8 = 0x06;

/// Defines the arch (usize) type for reading from a file;
pub const ARCH: u8 = 0x07;

/// Defines the iarch (isize) type for reading from a file;
pub const IARCH: u8 = 0x08;

/// Defines the CompTimePtr (usize) type for reading from a file;
pub const CTP: u8 = 0x0B;

// Composites

/// Defines the StaticStr (Arc<str>) type for reading from a file;
pub const STATICSTR: u8 = 0x09;

/// Defines the MutStr (String) type for reading from a file;
pub const MUTSTR: u8 = 0x0A;

/// Defines the Complex (f64, f64) type for reading from a file;
pub const COMPLEX: u8 = 0x0C;

/// Defines the ArrayStart (SqdArray) for reading an array from a file;
pub const ARRAYSTART: u8 = 0x2A;

/// Defines the ArrayEnd (SqdArray) for reading an array from a file;
pub const ARRAYEND: u8 = 0x2B;

/// Defines the FnObjStart (FnObj) for reading a Function-Object from a file;
pub const FNOBJSTART: u8 = 0x3A;

/// Defines the FnObjEnd (FnObj) for reading a Function-Object from a file;
pub const FNOBJEND: u8 = 0x3B;

// Compiler markers

/// Defines the CanJitStart (usize) marker for knowing portions of bytecode that can be JITtted.
pub const CANJITSTART: u8 = 0x1A;

/// Defines the CanJitStart (usize) marker for knowing portions of bytecode that can be JITtted.
pub const CANJITEND: u8 = 0x1B;

// String types

/// Defines a string of `2^8` (8 bits) max size for reading from a file.
pub const STRING8: u8 = 0x0F;

/// Defines a string of `2^16` (16 bits) max size for reading from a file.
pub const STRING16: u8 = 0x1F;

/// Defines a string of `2^32` (32 bits) max size for reading from a file.
pub const STRING32: u8 = 0x2F;

/// Defines a string of `2^64` (64 bits) max size for reading from a file.
pub const STRING64: u8 = 0x3F;

/// Defines a string of `2^128` (128 bits) max size for reading from a file.
pub const STRING128: u8 = 0x4F;
