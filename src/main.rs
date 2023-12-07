mod vminternals;
mod sqbinreader;
use byteorder;
use std::any::type_name;
use vminternals::VMStarter;
use std::{env, process};
use std::io::{Read, Seek};
use byteorder::{ReadBytesExt};
use crate::sqbinreader::FileReader;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}

fn main() {

    let mut vm = VMStarter::new(100);

    while vm.running == true {

    // vm.interpreter(
    //     (&[
    //         0x0A,
    //         0x0A,
    //         0x01,
    //         0x0A,
    //         0x0E,
    //         // 0x09,
    //         0x00,
    //     ]).to_vec(),
    //     &[
    //         Immediates::Integer(10),
    //         Immediates::Integer(12),
    //         Immediates::Null,
    //         Immediates::String(format!("\nLinha randômica!")),
    //         Immediates::Null,
    //         Immediates::Null,
    //     ],
    // );

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let fileread = FileReader::new(args[1].clone());

        vm.interpreter2(
            fileread
        );

    }else {
        eprintln!("\x1B[31m{}\x1b[0m", "File not specified!");
        process::exit(3);
    }

    }

    println!("Exiting...");
}
