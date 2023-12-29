#![cfg(test)]
use std::process;
use async_std::task;
use clap::Parser;
use crate::argsdef::{Args, string_to_bytesize};
use crate::errdef::MAXMEM_CONVERSION_ERR;
use crate::getup::get_update;
use crate::sqd_reader::sqdbin_reader::FileReader;
use crate::vm_internals::VMStarter;

#[test]
fn test_getup() {

    get_update();

}

#[test]
fn test_argsdef() {

    let args = Args::try_parse();

    match args {
        Ok(arguments) => {
            println!("{:?}", arguments);
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    let realmaxmem = "sdgsdfgsdfs".to_string();

    match string_to_bytesize(realmaxmem) {
        Ok(mem) => {
            assert_eq!(mem, 100usize);
        }
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
        }
    }

    let realmaxmem = "sdgsdfgsdfsB".to_string();

    match string_to_bytesize(realmaxmem) {
        Ok(mem) => {
            assert_eq!(mem, 100usize);
        }
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
        }
    }

    let realmaxmem = "sdgsdfgsdfsKB".to_string();

    match string_to_bytesize(realmaxmem) {
        Ok(mem) => {
            assert_eq!(mem, 100usize);
        }
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
        }
    }

    let realmaxmem = "sdgsdfgsdfsMB".to_string();

    match string_to_bytesize(realmaxmem) {
        Ok(mem) => {
            assert_eq!(mem, 100usize);
        }
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
        }
    }

    let realmaxmem = "sdgsdfgsdfsGB".to_string();

    match string_to_bytesize(realmaxmem) {
        Ok(mem) => {
            assert_eq!(mem, 100usize);
        }
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
        }
    }

    let realmaxmem = "100B".to_string();

    match string_to_bytesize(realmaxmem) {
        Ok(mem) => {
            assert_eq!(mem, 100usize);
        }
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
            process::exit(MAXMEM_CONVERSION_ERR);
        }
    }

    let realmaxmem = "100KB".to_string();

    match string_to_bytesize(realmaxmem) {
        Ok(mem) => {
            assert_eq!(mem, 102400usize);
        }
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
            process::exit(MAXMEM_CONVERSION_ERR);
        }
    }

    let realmaxmem = "1GB".to_string();

    match string_to_bytesize(realmaxmem) {
        Ok(mem) => {
            assert_eq!(mem, 1073741824usize);
        }
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
            process::exit(MAXMEM_CONVERSION_ERR);
        }
    }

}

#[test]
fn test_main_bin() {

    let mut fileread: Option<FileReader> = None;
    let realmaxmem = "100MB".to_string();
    let maxmem;

    match string_to_bytesize(realmaxmem) {
        Ok(mem) => {
            assert_eq!(mem, 104857600usize);
            maxmem = mem;
        }
        Err(err) => {
            eprintln!("\x1B[31m{}\x1b[0m", err);
            process::exit(MAXMEM_CONVERSION_ERR);
        }
    }

    let bin = "./examples/coveragetest".to_string();

    fileread = Some(FileReader::new(bin, false, false).unwrap());

    let mut vm = VMStarter::new(maxmem, 20);
    // dev_print!("{:?}", vm);

    if let Some(fileread) = fileread {
        while vm.running {
            vm.interpreter(fileread.clone());
        }
    }

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

    dev_print!("Exiting...");

}
