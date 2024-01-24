/// If no argument is passed this error code is thrown.
pub const ARG_MISSING_ERR: i32 = 3;

/// Error that happens when the argument passed for maxmem fails to be converted.
pub const MAXMEM_CONVERSION_ERR: i32 = 4;

/// Error while evaluating metadata from file.
pub const METADATA_ERR: i32 = 5;

/// Error while reading file.
pub const FILE_DATA_ERR: i32 = 6;

/// Error that happens when trying to allocate something to the heap.
pub const HEAP_ALLOC_ERR: i32 = 2;

/// Error that happens when trying to check for updates.
/// Error out while trying to get latest update or while trying to parse JSON.
pub const UPDATE_CHECK_ERR: i32 = 7;

pub const FEATURE_ERR: i32 = 10;