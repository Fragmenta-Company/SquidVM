/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
pub const TARGET: &str = "Windows x86_64";

/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(target_os = "linux")]
#[cfg(target_arch = "x86_64")]
pub const TARGET: &str = "Linux x86_64";

/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(target_os = "linux")]
#[cfg(target_arch = "aarch64")]
pub const TARGET: &str = "Linux ARMv8";

/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(target_os = "linux")]
#[cfg(target_arch = "arm")]
pub const TARGET: &str = "Linux ARMv7";

/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(not(target_arch = "x86_64"))]
#[cfg(not(target_arch = "aarch64"))]
#[cfg(not(target_arch = "arm"))]
pub const TARGET: &str = "other platform";

/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(not(target_os = "windows"))]
#[cfg(not(target_os = "linux"))]
pub const TARGET: &str = "other";
