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

/// Defines the null type while reading from file.
pub const NULL: u8 = 0x00;

/// Defines the bool type while reading from file.
pub const BOOL: u8 = 0x01;

/// Defines the integer type while reading from file.
pub const INTEGER: u8 = 0x02;

/// Defines the unsigned integer type while reading from file.
pub const UINTEGER: u8 = 0x03;

/// Defines the float type while reading from file.
pub const FLOAT: u8 = 0x04;

/// Defines the 8bit string type while reading from file.
pub const STRING8: u8 = 0x0F;

/// Defines the 16bit string  type while reading from file.
pub const STRING16: u8 = 0x1F;

/// Defines the 32bit string  type while reading from file.
pub const STRING32: u8 = 0x2F;

/// Defines the 64bit string  type while reading from file.
pub const STRING64: u8 = 0x3F;

/// Defines the 128bit string  type while reading from file.
pub const STRING128: u8 = 0x4F;
