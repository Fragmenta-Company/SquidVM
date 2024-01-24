/// ## VM, interpreter and instruction implementation
#[macro_use]
pub mod vm;

/// ## VM's heap and garbage collector implementation
pub mod heap;

/// ## VM's Immediate types and other implementations
pub mod immediates;

/// ## VM's repository implementation (Here lies variables as heap pointers).
/// Can be used for global variables.
pub mod repository;

/// ## VM's stack implementation
pub mod stack;

/// ## Threads implementation
pub mod vm_threads;

pub mod windowing;

pub use windowing::*;

pub use repository::*;

pub use heap::*;
// pub use immediates::*;
pub use stack::*;
pub use vm::*;
