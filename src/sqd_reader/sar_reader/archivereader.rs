use crate::vm_internals::immediates::Immediates;
use std::io::Read;

/// Holds all the instructions and data that
/// the VM will use in order to function properly.
#[derive(Clone)]
pub struct ArchiveReader {
    /// Contains the instructions that the VM will utilize.
    pub instructions: Vec<u8>,

    /// Contains all the data the instructions will use to work.
    pub data: Vec<Immediates>,
}

#[cfg(feature = "bundle")]
#[allow(dead_code)]
const BYTECODE: &'static [u8] = include_bytes!("../../../examples/addnprint.sqdbin.zip");

#[cfg(feature = "bundle")]
impl ArchiveReader {
    pub fn new() {
        let bytes = std::io::Cursor::new(BYTECODE);

        let something = zip::ZipArchive::new(bytes).unwrap();

        let mut idk = something.clone();

        let mut idk2 = idk.by_name("addnprint.sqdbin").unwrap();

        let mut buf = Vec::new();

        idk2.read_to_end(&mut buf).unwrap();

        println!("Buffer: {:X?}", buf);

        // println!("{something:?}");
    }
}
