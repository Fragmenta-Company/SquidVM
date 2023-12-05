mod vminternals;

// use crate::vminternals::VMHeap;
use std::any::type_name;
use vminternals::immediates::Immediates;
use vminternals::VMStarter;
// use std::env;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}

fn main() {
    // println!("{:?}", b"idk");

    // let mut heap = VMHeap::new(10);
    //
    // heap.add_var(0x00, Immediates::Integer(27));
    //
    // let var = heap.get_var(0x00);
    //
    // if let Immediates::Integer(i) = var {
    //
    //     println!("{}", *i)
    //
    // }

    let mut vm = VMStarter::new(100);

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
