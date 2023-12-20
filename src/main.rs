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

/// Defines the program macros
#[macro_use]
mod macrodefs;

/// Contains the sqdbin binary files reader implementation
mod sqdbinreader;

/// Contains the entirety of the VM internal implementation
mod vminternals;

/// Used for cli arguments definintion
mod argsdef;

/// Defines the exit codes/error codes that the program will throw.
mod errdef;
/// Module used for reading the Squid ARchives
mod sarreader;

/// Defines all the instructions.
mod instructiondefs;
/// Defines the target that show when using `./squid-vm(.exe) --version`
mod targetdef;

use argsdef::*;
use clap::Parser;
use errdef::*;
use sqdbinreader::FileReader;
use std::process;
use targetdef::*;
use vminternals::VMStarter;

/// Get arguments from the command and creates a VMStarter object.
/// Run vm.interpreter in loop while vm is running.
/// File is read and converted to VM readble objects before the interpreter starts.
fn main() {
    let mut fileread: Option<FileReader> = None;
    let maxmem;
    let mut bin: Option<String> = None;
    let mut sar: Option<String> = None;

    let args = Args::parse();

    if args.version {
        println!("{} {} for {}", "SquidVM", env!("CARGO_PKG_VERSION"), TARGET);
        process::exit(0);
    }

    match string_to_bytesize(args.maxmem) {
        Ok(mem) => {
            maxmem = mem;
        }
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
            process::exit(MAXMEM_CONVERSION_ERR);
        }
    }

    // Checks if is there a binary file specificed.
    if let Some(binary) = args.bin {
        bin = Some(binary)
    } else if let Some(archive) = args.sar {
        sar = Some(archive)
    }

    if bin.is_none() && sar.is_none() {
        eprintln!("\x1B[31mNo option specified!\x1b[0m");
        process::exit(ARG_MISSING_ERR);
    }

    if let Some(bin) = bin {
        fileread = Some(FileReader::new(bin, args.binver));
    } else if let Some(_sar) = sar {
    }

    let mut vm = VMStarter::new(maxmem, args.repo_size);
    // dev_print!("{:?}", vm);

    if let Some(fileread) = fileread {
        while vm.running {
            vm.interpreter(fileread.clone());
        }
    }

    dev_print!("Exiting...");
}
