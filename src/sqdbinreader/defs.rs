/// .sqdbin file extension.
pub const SQDBIN: &str = ".sqdbin";

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
