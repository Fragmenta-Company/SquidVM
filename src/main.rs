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
//!     todo!(); //in this project

#![warn(missing_docs)]

/// Contains the sqdbin binary files reader implementation
mod sqdbinreader;

/// Contains the entirety of the VM internal implementation
mod vminternals;
use crate::sqdbinreader::FileReader;
use std::{env, process};
use vminternals::VMStarter;

/// Get arguments from the command and creates a VMStarter object.
/// Run vm.interpreter in loop while vm is running.
/// File is read and converted to VM readble objects before the interpreter starts.
fn main() {
    let mut vm = VMStarter::new(512, 10);

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let fileread = FileReader::new(args[1].clone());
        while vm.running {
            vm.interpreter(fileread.clone());
        }
    } else {
        eprintln!("\x1B[31mFile not specified!\x1b[0m");
        process::exit(3);
    }

    println!("Exiting...");
}
