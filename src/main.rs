mod sqdbinreader;
mod vminternals;
use crate::sqdbinreader::FileReader;
// use crate::vminternals::GetLength;
use byteorder;
// use byteorder::ReadBytesExt;
use std::any::type_name;
use std::io::{Read, Seek};
use std::{env, process};
use vminternals::VMStarter;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}

fn main() {
    let mut vm = VMStarter::new(512, 10);

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let fileread = FileReader::new(args[1].clone());
        while vm.running == true {
            vm.interpreter2(fileread.clone());
        }
    } else {
        eprintln!("\x1B[31m{}\x1b[0m", "File not specified!");
        process::exit(3);
    }

    println!("Exiting...");
}
