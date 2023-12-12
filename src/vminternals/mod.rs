/// ## VM's heap and garbage collector implementation
pub mod heap;

/// ## VM's Immediate types and other implementations
pub mod immediates;

/// ## VM's repository implementation (Here lies variables as heap pointers).
/// Can be used for global variables.
pub mod repository;

/// ## VM's stack implementation
pub mod stack;

/// ## VM, interpreter and instruction implementation
pub mod vm;

pub use repository::*;

pub use heap::*;
// pub use immediates::*;
pub use stack::*;
pub use vm::*;
