/// Halts the VM/thread.
pub const HALT: u8 = 0x00;

/// Adds two integers from the stack.
pub const I_ADD: u8 = 0x01;

/// Subtracts two integers from the stack.
pub const I_SUB: u8 = 0x02;

/// Multiplies two integers from the stack.
pub const I_MUL: u8 = 0x03;

/// Divides two integers from the stack.
pub const I_DVD: u8 = 0x04;

/// Divides two integers from the stack (and returns integer).
pub const F_I_DVD: u8 = 0x05;

/// Adds two floats from the stack.
pub const F_ADD: u8 = 0x06;

/// Subtracts two floats from the stack.
pub const F_SUB: u8 = 0x07;

/// Multiplies two floats from the stack.
pub const F_MUL: u8 = 0x08;

/// Divides two floats from the stack.
pub const F_DVD: u8 = 0x09;

/// Puts data to stack.
pub const PDTS: u8 = 0x0A;

/// Puts data from stack.
pub const PDFS: u8 = 0x0B;

/// Jumps from data.
pub const JMPFD: u8 = 0x0C;

/// Jumps from stack.
pub const JMPFS: u8 = 0x0D;

/// Pops and prints from stack.
pub const PRTFS: u8 = 0x0E;

/// Prints from data.
pub const PRTFD: u8 = 0x0F;

/// Take two integers from the stack and perform exponentiation.
pub const I_EXP: u8 = 0x10;

/// Take two floats from the stack and perform exponentiation.
pub const F_EXP: u8 = 0x11;

/// Take one float and one integer from the stack and perform exponentiation.
pub const F_I_EXP: u8 = 0x12;

/// Add variable pointer.
pub const AVP: u8 = 0x15;

/// Debug variable from data.
pub const D_VFD: u8 = 0x16;

/// Debug variable from stack.
pub const D_VFS: u8 = 0x17;

/// Creates new task.
pub const NTASK: u8 = 0x19;

/// Creates new thread.
pub const NTHRD: u8 = 0x1A;

/// Panics the current thread
pub const PANIC: u8 = 0x1B;

pub const PEEK: u8 = 0x1C;

pub const SWAP: u8 = 0x1D;

pub const EQUALS: u8 = 0x1E;

pub const LESSTHAN: u8 = 0x1F;

pub const GREATERTHAN: u8 = 0x20;

pub const AND: u8 = 0x21;

pub const OR: u8 = 0x22;
