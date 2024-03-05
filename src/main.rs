//! <p style="text-align: center;">
//!     <img src="https://d1qrumake2q7xa.cloudfront.net/squid-vm.svg"
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
//! ```rust
//! todo!(); //in this project.
//! ```
#![warn(missing_docs)]

/// Changes from SquidVM to SVDK when feature devkit is enabled on compile time.
#[cfg(feature = "devkit")]
const VM_NAMING_CONVENTION: &str = "SquidVM Development Kit";

/// Changes from SquidVM to SVDK when feature devkit is enabled on compile time.
#[cfg(not(feature = "devkit"))]
const VM_NAMING_CONVENTION: &str = "SquidVM";

/// Defines the program macros
#[macro_use]
mod macrodefs;

/// All logic used to run binary or sar files.
mod sqd_reader;

/// Contains the entirety of the VM internal implementation.
mod vm_internals;

/// Used for cli arguments definintion.
mod argsdef;

/// Defines the exit codes/error codes that the program will throw.
mod errdef;

/// Module used for getting updates from the GitHub repo.
mod getup;
/// Defines all the instructions.
mod instructiondefs;
/// Defines the target that show when using
/// ```shell
/// ./squid-vm(.exe) --version
/// ```.
mod targetdef;
mod tests;

#[cfg(feature = "default")]
use argsdef::*;

use crate::vm_internals::PrintMessage;
#[cfg(feature = "green-threads")]
use async_std::task;
#[cfg(feature = "default")]
use clap::Parser;
use errdef::*;
use sqd_reader::sqdbin_reader::FileReader;
use std::process;
use targetdef::*;
use vm_internals::VMStarter;

#[cfg(feature = "default")]
/// Contains tools for checking updates, getting current version and others.
#[cfg(not(test))]
fn version_args(args: &Args) {
    #[cfg(feature = "check-update")]
    if args.check_updates {
        println!("Current version: {}", env!("CARGO_PKG_VERSION"));

        getup::get_update().iter().rev().for_each(move |string| {
            println!("{string}");
        });

        process::exit(0);
    }

    #[cfg(not(feature = "check-update"))]
    if args.check_updates {
        eprintln!("'check-update' Feature not enabled!");

        process::exit(FEATURE_ERR);
    }

    if args.version {
        dev_print!("---- SVDK ---- ---- SVDK ---- SVDK ---- ---- SVDK ----");
        println!(
            "{} {} for {}",
            VM_NAMING_CONVENTION,
            env!("CARGO_PKG_VERSION"),
            TARGET
        );
        dev_print!("---- SVDK ---- ---- SVDK ---- SVDK ---- ---- SVDK ----");
        process::exit(0);
    }
}

#[cfg(not(feature = "default"))]
fn main() {
    dev_print!("Exiting...");
}

#[cfg(feature = "default")]
/// Get arguments from the command and creates a VMStarter object.
/// Run vm.interpreter in loop while vm is running.
/// File is read and converted to VM readble objects before the interpreter starts.
#[cfg(not(test))]
fn main() {
    #[cfg(feature = "bundle")]
    sqd_reader::sar_reader::archivereader::ArchiveReader::new();

    let mut fileread: Option<FileReader> = None;
    let mut bin: Option<String> = None;
    let mut sar: Option<String> = None;

    let args = Args::parse();

    version_args(&args);

    let maxmem = match string_to_bytesize(args.maxmem) {
        Ok(mem) => mem,
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
            process::exit(MAXMEM_CONVERSION_ERR);
        }
    };

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
        fileread = Some(FileReader::new(bin, args.binver, args.force_newer_bin).unwrap());
    } else if let Some(_sar) = sar {
    }

    let mut vm = VMStarter::new(maxmem, args.repo_size, args.stack_size);

    if let Some(fileread) = fileread {
        while vm.running {
            vm.interpreter(fileread.clone());
        }
    }

    #[cfg(feature = "green-threads")]
    if vm.task_handlers.len() > 0 {
        task::block_on(async {
            for task in vm.task_handlers {
                match task.await {
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("\x1B[41m{}\x1B[0m", err);
                    }
                };
            }
        });
    }

    if vm.thread_handlers.len() > 0 {
        for task in vm.thread_handlers {
            match task.join().unwrap() {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("\x1B[41m{}\x1B[0m", err);
                }
            };
        }
    }

    vm.print_sender.send(PrintMessage::End).unwrap();
    vm.print_handler.join().unwrap();

    dev_print!("Exiting...");
}
