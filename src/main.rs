mod vminternals;

use vminternals::stack::*;
use vminternals::VMStarter;
use std::any::type_name;
use std::env;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>())
}

fn main() {

    // env::set_var("RUST_BACKTRACE", "1");

    // let mut s = VMStack::new(200);
    //
    // println!("{}", s.check_empty());
    //
    // s.push(Immediates::Integer(10));
    //
    // println!("{}", s.check_empty());
    //
    // let idk = s.pop();
    //
    // if let Immediates::Integer(i) = idk {
    //
    //     println!("{}", i);
    //
    // }else {
    //     panic!("[ PANIC! ]: Was not an Immediate::Integer()");
    // }

    let mut vm = VMStarter::new(100);

    // println!("Length before push: {}", vm.get_length());
    //
    // println!("Length after push: {}", vm.get_length());

    vm.interpreter(&[
        0x0A,
        0x0A,
        0x01,
        0x0A,
        0x0E,
        // 0x09,
        0x00
    ], &[
        Immediates::Integer(10),
        Immediates::Integer(12),
        Immediates::Null,
        Immediates::String(format!("Random String!")),
        Immediates::Null,
        Immediates::Null,
    ]);

}


// Test code, move to main to test:

// let mut counter = 0;

// loop {
//
//     if vm.running == false {
//         println!("Exiting...");
//         break;
//     }
//
//     vm.instructor();
//
//     if vm.check_empty() == false {
//
//         counter += 1;
//
//         println!("Length: {}", vm.get_length());
//
//         println!("Tempos 2.0: {}", counter);
//
//         let popped = vm.pop();
//
//         if let Immediates::Integer(i) = popped {
//
//             print_type_of(&i);
//
//             println!("{}", i);
//
//         }
//
//         if let Immediates::Float(f) = popped {
//
//             print_type_of(&f);
//
//             println!("{}", f);
//
//         }
//
//     }
//
// }