use crate::vm_internals::immediates::Immediates;

/// Holds all the instructions and data that
/// the VM will use in order to function properly.
#[derive(Clone)]
pub struct ArchiveReader {
    /// Contains the instructions that the VM will utilize.
    pub instructions: Vec<u8>,

    /// Contains all the data the instructions will use to work.
    pub data: Vec<Immediates>,
}

