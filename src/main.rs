mod vminternals;

use std::any::type_name;
use vminternals::stack::*;
use vminternals::VMStarter;
// use std::env;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}

fn main() {
    let mut vm = VMStarter::new();

    vm.interpreter(
        &[
            0x0A,
            0x0A,
            0x01,
            0x0A,
            0x0E,
            // 0x09,
            0x00,
        ],
        &[
            Immediates::Integer(10),
            Immediates::Integer(12),
            Immediates::Null,
            Immediates::String(format!("Random String!")),
            Immediates::Null,
            Immediates::Null,
        ],
    );

    println!("Exiting...");
}