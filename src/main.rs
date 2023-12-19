//! <p style="text-align: center;">
//!     <img src="https://d1drfx3idpovxr.cloudfront.net/squid-vm.svg"
//!     alt="SquidVM Logo" width="250px" />
//! </p>
//!
//! SquidVM is a VM made in Rust, having a high performant architecture.
//!
//! Considerations:
//! - The VM is in the process of being released as a full version (1.0.0)!
//! - It is not production ready!
//! But you can use it for your projects if you want.
//! - You can fork the VM, it's free for everyone!
//! - All the collaborations made for the main project will need to have the same license!
//!
//! I still have lots of things
//!
//!     todo!(); //in this project.

#![warn(missing_docs)]

#[cfg(feature = "devkit")]
macro_rules! dev_print {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

/// Show debug info if devkit feature is enabled in compile time.
#[cfg(not(feature = "devkit"))]
macro_rules! dev_print {
    ($($arg:tt)*) => {};
}

/// Contains the sqdbin binary files reader implementation
mod sqdbinreader;

/// Contains the entirety of the VM internal implementation
mod vminternals;

/// Module used for reading the Squid ARchives
mod sarreader;

use crate::sqdbinreader::FileReader;
use clap::Parser;
use std::process;
use vminternals::VMStarter;
// use crate::vminternals::immediates::{Immediates, Serialize};
// use crate::vminternals::VMHeap;

/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
const TARGET: &str = "Windows x86_64";

/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(target_os = "linux")]
#[cfg(target_arch = "x86_64")]
const TARGET: &str = "Linux x86_64";

/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(target_os = "linux")]
#[cfg(target_arch = "aarch64")]
const TARGET: &str = "Linux ARMv8";

/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(target_os = "linux")]
#[cfg(target_arch = "arm")]
const TARGET: &str = "Linux ARMv7";

/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(not(target_arch = "x86_64"))]
#[cfg(not(target_arch = "aarch64"))]
#[cfg(not(target_arch = "arm"))]
const TARGET: &str = "other platform";

/// Set the target constant to show when using `./squid-vm(.exe) --version`.
#[cfg(not(target_os = "windows"))]
#[cfg(not(target_os = "linux"))]
const TARGET: &str = "other";

/// Converts strings with postfixes (GB, MB, KB or B) into a value in bytes
fn string_to_bytesize(string: String) -> Result<usize, &'static str> {
    // dev_print!("Before pop: {}", string);

    if string.ends_with("GB") {
        let mut gb = string;

        gb.pop().unwrap();
        gb.pop().unwrap();

        // dev_print!("After pop: {}", gb);

        if let Ok(gb) = gb.parse::<f64>() {
            Ok((gb * (1024 * 1024 * 1024) as f64) as usize)
        } else {
            Err("Failed to parse numeric part")
        }
    } else if string.ends_with("MB") {
        let mut mb = string;

        mb.pop().unwrap();
        mb.pop().unwrap();

        // dev_print!("After pop: {}", mb);

        if let Ok(mb) = mb.parse::<f64>() {
            Ok((mb * (1024 * 1024) as f64) as usize)
        } else {
            Err("Failed to parse numeric part")
        }
    } else if string.ends_with("KB") {
        let mut kb = string;

        kb.pop().unwrap();
        kb.pop().unwrap();

        // dev_print!("After pop: {}", kb);

        if let Ok(kb) = kb.parse::<f64>() {
            Ok((kb * 1024f64) as usize)
        } else {
            Err("Failed to parse numeric part")
        }
    } else if string.ends_with("B") {
        let mut b = string;

        b.pop().unwrap();

        // dev_print!("After pop: {}", b);

        if let Ok(b) = b.parse::<usize>() {
            Ok(b)
        } else {
            Err("Failed to parse numeric part")
        }
    } else {
        Err("Number need a postfix -> GB, MB, KB or B")
    }
}

/// Argument Parser
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// Binary File Input | Don't need extension (.sqdbin)
    #[arg(short, long, value_name = "FILE", conflicts_with = "sar")]
    bin: Option<String>,

    /// Squid ARchive File Input | Don't need extension (.sar)
    #[arg(short, long, value_name = "FILE", conflicts_with = "bin")]
    sar: Option<String>,

    /// Max Memory Allocated for the heap | Postfixes: GB, MB, KB, B
    #[arg(short, long, value_name = "SIZE", default_value = "512MB")]
    maxmem: String,

    /// The repository is used for saving global variables easily
    #[arg(short, long, value_name = "SIZE", default_value = "20")]
    repo_size: usize,

    /// Check the binary version for binaries with metadata included | Works with bin or sar
    #[arg(
        long,
        value_name = "FILE",
        requires = "bin",
        requires = "sar",
        short = 'v'
    )]
    binver: bool,

    /// Shows the SquidVM version | SquidVM |major|.|minor|.|patch|-|details| for |OS| |arch|
    #[arg(long, short = 'V')]
    version: bool,
}

/// Get arguments from the command and creates a VMStarter object.
/// Run vm.interpreter in loop while vm is running.
/// File is read and converted to VM readble objects before the interpreter starts.
fn main() {
    // let mut heap_test = VMHeap::new(123);
    //
    // let alloc = heap_test.malloc(Immediates::Float(13.4));
    //
    // let smth = heap_test.get_obj(alloc.index);
    //
    // println!("Obj: {:?}", smth);
    // println!("Data: {:?}", f64::from_bits(u64::from_le_bytes(smth.data.as_slice().try_into().unwrap())));
    // println!("Alloc: {:?}", alloc);

    let mut fileread: Option<FileReader> = None;

    let args = Args::parse();

    if args.version {
        println!("{} {} for {}", "SquidVM", env!("CARGO_PKG_VERSION"), TARGET);
        process::exit(0);
    }

    let maxmem;

    match string_to_bytesize(args.maxmem) {
        Ok(mem) => {
            maxmem = mem;
        }
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
            process::exit(4);
        }
    }

    // dev_print!("maxmem: {}", maxmem);
    // dev_print!("reposize: {}", args.reposize);
    let mut vm = VMStarter::new(maxmem, args.repo_size);
    dev_print!("{:?}", vm);

    // let args: Vec<String> = env::args().collect();

    let mut bin: Option<String> = None;
    let mut sar: Option<String> = None;

    // Checks if is there a binary file specificed.
    if let Some(binary) = args.bin {
        bin = Some(binary)
    } else if let Some(archive) = args.sar {
        sar = Some(archive)
    }

    if bin.is_none() && sar.is_none() {
        eprintln!("\x1B[31mFile not specified!\x1b[0m");
        process::exit(3);
    }

    if let Some(bin) = bin {
        fileread = Some(FileReader::new(bin, args.binver));
    } else if let Some(_sar) = sar {
    }

    if let Some(fileread) = fileread {
        while vm.running {
            vm.interpreter(fileread.clone());
        }
    }

    dev_print!("Exiting...");
}
