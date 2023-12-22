use crate::errdef::*;
use crate::instructiondefs::*;
use crate::sqdbinreader::defs::*;
use crate::vminternals::immediates::Immediates::{
    self, Boolean, Float, Integer, Null, String as TypeString, UInteger,
};
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom};
use std::{fs, process};

/// Holds all the instructions and data that
/// the VM will use in order to function properly.
#[derive(Clone)]
pub struct FileReader {
    /// Contains the instructions that the VM will utilize.
    pub instructions: Vec<u8>,

    /// Contains all the data the instructions will use to work.
    pub data: Vec<Immediates>,
}

/// Converts vector of bytes into UTF8 compatible strings.
fn to_string(string: Vec<u8>) -> String {
    match String::from_utf8(string) {
        Ok(uf8string) => uf8string,
        Err(e) => {
            panic!("INVALID FILE DATA! {}", e);
        }
    }
}

#[allow(unused_assignments)]
/// FileReader struct implementation
impl FileReader {
    /// Reads file_location and gets file contents.
    /// The binary file is converted into a VM readble form,
    /// Thus leading to two objects:
    /// * instructions: Contains the instructions for the VM to run;
    /// * data: Contains the data that each instruction will use.
    ///
    /// For example, if the instruction is for adding to the stack,
    /// it will probably contain some data info, like Integers,
    /// Strings, Floats or even Null values.
    pub fn new(mut file_location: String, filearg: bool, force_newer_ver: bool) -> FileReader {
        if file_location.ends_with('\\') || file_location.ends_with('/') {
            file_location.pop();
        }

        if !file_location.ends_with(SQDBIN) {
            file_location.push_str(SQDBIN);
        }

        let mut instructions: Vec<u8> = Vec::new();
        let mut data: Vec<Immediates> = Vec::new();
        let file = File::open(file_location.clone());
        let mut file = match file {
            Ok(file) => file,
            Err(error) => {
                eprintln!("\x1B[31m{}\x1b[0m", error);
                process::exit(FILE_DATA_ERR);
            }
        };

        let mut offset = 0x00;
        let mut counter = 0;
        let filelength = fs::metadata(file_location)
            .expect("INVALID FILE METADATA")
            .len();
        // println!("{filelength}");

        loop {
            let crsr = file.seek(SeekFrom::Start(offset)).unwrap();

            #[allow(unused_variables)]
            fn set_crsr(mut crsr: u64, mut file: &File, offset: &u64) {
                crsr = match file.seek(SeekFrom::Start(*offset)) {
                    Ok(crsr) => crsr,
                    Err(err) => {
                        eprintln!("\x1B[31m{}\x1b[0m", err);
                        process::exit(FILE_DATA_ERR);
                    }
                }
            }

            let mut buffer = [0u8; 2];

            match file.read_exact(&mut buffer) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("\x1B[31m{}\x1b[0m", e);
                    process::exit(FILE_DATA_ERR);
                }
            };

            if counter < 1 {
                fn error_handler<T>(res: Result<T, Error>) -> T {
                    match res {
                        Ok(byte) => byte,
                        Err(err) => {
                            dev_print!("Error: {:?}", err);
                            eprintln!("\x1B[31mINVALID FILE METADATA!\x1b[0m");
                            process::exit(METADATA_ERR);
                        }
                    }
                }

                // Check if file has metadata
                match buffer[0] {
                    // File has metadata
                    1 => {
                        offset += METADATA_IDENTIFIER_BYTE;

                        set_crsr(crsr, &file, &offset);

                        let major = error_handler(file.read_u32::<LittleEndian>());

                        offset += MAJOR_SIZE;

                        let minor = error_handler(file.read_u16::<LittleEndian>());

                        let wrong_ver = if !force_newer_ver {
                            // Binary major is higher than VM's
                            if major > VM_MAJOR.parse().unwrap() {
                                true
                            } else if minor > VM_MINOR.parse().unwrap()
                                && major == VM_MAJOR.parse::<u32>().unwrap()
                            {
                                // Binary major is equal to VM's, but minor is higher
                                true
                            } else {
                                // Binary have correct version
                                false
                            }
                        } else {
                            // Force binary to run
                            false
                        };

                        if filearg || wrong_ver {
                            offset += MINOR_PATCH_SIZE;

                            let patch = error_handler(file.read_u16::<LittleEndian>());

                            offset += MINOR_PATCH_SIZE;

                            let details = error_handler(file.read_u8());

                            let mut counter: u32 = 0;
                            let mut byte_string = vec![0; 6];

                            while counter < COMPILER_NAME_SIZE {
                                byte_string.push(error_handler(file.read_u8()));
                                offset += 1;
                                counter += 1;
                            }

                            offset += 1;

                            if wrong_ver && !filearg {
                                let details = match details {
                                    0 => "release",
                                    1 => "alpha",
                                    2 => "beta",
                                    _ => "unknown",
                                };

                                eprintln!("\x1B[41mBinary was compiled for a more recent version of the VM!\x1B[0m");
                                println!(
                                    "\x1B[32mCurrent VM version: {}\x1B[0m",
                                    env!("CARGO_PKG_VERSION")
                                );
                                let version = format!(
                                    "{}.{}.{}-{} and up until next major",
                                    major, minor, patch, details
                                );
                                println!(
                                    "\x1B[33mBinary was compiled for version {}\x1B[0m",
                                    version
                                );
                                process::exit(FILE_DATA_ERR);
                            }

                            let details = match details {
                                0 => "Release",
                                1 => "Alpha",
                                2 => "Beta",
                                _ => "Unknown",
                            };

                            let version = format!(
                                "Compatible with version {}.{}.{}-{} and up until next major",
                                major, minor, patch, details
                            );

                            println!("{version}");
                            println!("\x1B[31mIn alpha and beta versions the VM will change a lot, so most things will change.");
                            println!("Binaries for SquidVM 0.7.0-alpha and up will not be compatible with old versions\x1b[0m");

                            let compiler = match String::from_utf8(byte_string) {
                                Ok(string) => string,
                                Err(err) => {
                                    dev_print!("Error: {:?}", err);
                                    eprintln!("\x1B[31mINVALID FILE METADATA!\x1b[0m");
                                    process::exit(METADATA_ERR);
                                }
                            };

                            println!("Compiled with: {compiler}");
                        } else {
                            offset += 27;
                            set_crsr(crsr, &file, &offset);
                        }

                        file.read_exact(&mut buffer).expect("INVALID FILE DATA!");
                    }
                    // File does not have metadata
                    _ => {
                        println!("\x1B[44mFile doesn't include metadata!\x1b[0m");
                        println!("\x1B[41mCompatibility can't be guaranteed.\x1b[0m");
                        println!(
                            "\x1B[41mThis may affect the proper functioning of the program.\x1b[0m"
                        );
                    }
                }

                if filearg {
                    process::exit(0);
                }
            }

            // Jumps the header if the counter is less than one and
            // if the file has a 0x01 as it's first byte.
            if counter < 1 && buffer[0] == METADATA_IDENTIFIER_BYTE as u8 {
                offset += HEADER_SIZE;

                set_crsr(crsr, &file, &offset);

                file.read_exact(&mut buffer).expect("INVALID FILE DATA!");
            }

            if counter > 1 && crsr == 0 {
                break;
            }

            dev_print!(
                "Cursor: {}, Buffer: {:?}, Counter: {}",
                crsr,
                buffer,
                counter
            );
            //
            // println!("Idk");

            match buffer[0] {
                HALT => {
                    // println!("Idk");
                    instructions.push(0x00);
                    data.push(Null);
                    break;
                }
                I_ADD => {
                    instructions.push(0x01);
                    data.push(Null);
                    offset += 1;
                }
                I_SUB => {
                    instructions.push(0x02);
                    data.push(Null);
                    offset += 1;
                }
                I_MUL => {
                    instructions.push(0x03);
                    data.push(Null);
                    offset += 1;
                }
                I_DVD => {
                    instructions.push(0x04);
                    data.push(Null);
                    offset += 1;
                }
                F_I_DVD => {
                    instructions.push(0x05);
                    data.push(Null);
                    offset += 1;
                }
                F_ADD => {
                    instructions.push(0x06);
                    data.push(Null);
                    offset += 1;
                }
                F_SUB => {
                    instructions.push(0x07);
                    data.push(Null);
                    offset += 1;
                }
                F_MUL => {
                    instructions.push(0x08);
                    data.push(Null);
                    offset += 1;
                }
                F_DVD => {
                    instructions.push(0x09);
                    data.push(Null);
                    offset += 1;
                }
                PDTS => {
                    instructions.push(buffer[0]);
                    match buffer[1] {
                        0x00 => {
                            // Null Type
                            data.push(Null);
                            offset += 2;
                        }
                        0x01 => {
                            // Boolean type
                            file.read_exact(&mut buffer).expect("INVALID FILE DATA!");
                            data.push(Boolean(if buffer[1] == 1 {
                                true
                            } else if buffer[1] == 0 {
                                false
                            } else {
                                panic!("INVALID FILE DATA!");
                            }));
                            offset += 2;
                        }
                        0x02 => {
                            // Integer type
                            offset += 2;
                            data.push(Integer(
                                file.read_i64::<LittleEndian>().expect("INVALID FILE DATA!"),
                            ));
                            offset += 8;
                        }
                        0x03 => {
                            // Unsigned Integer type
                            offset += 2;
                            data.push(UInteger(
                                file.read_u64::<LittleEndian>().expect("INVALID FILE DATA!"),
                            ));
                            offset += 8;
                        }
                        0x04 => {
                            // Float type
                            offset += 2;
                            data.push(Float(
                                file.read_f64::<LittleEndian>().expect("INVALID FILE DATA!"),
                            ));
                            offset += 8;
                        }
                        0x0F => {
                            // 8bit type
                            offset += 2;
                            let int = file.read_u8().expect("INVALID FILE DATA!");
                            let mut counter = 0;
                            offset += 1;
                            let mut byte_string: Vec<u8> = Vec::with_capacity(255);
                            while counter < int {
                                byte_string.push(file.read_u8().expect("INVALID FILE DATA!"));
                                offset += 1;
                                counter += 1;
                            }

                            data.push(TypeString(to_string(byte_string)));
                        }
                        0x1F => {
                            // 16bit type
                            offset += 2;
                            let int = file.read_u16::<LittleEndian>().expect("INVALID FILE DATA!");
                            let mut counter = 0;
                            offset += 2;
                            let mut byte_string: Vec<u8> = Vec::with_capacity(2usize.pow(16));
                            while counter < int {
                                byte_string.push(file.read_u8().expect("INVALID FILE DATA!"));
                                offset += 1;
                                counter += 1;
                            }
                            data.push(TypeString(to_string(byte_string)));
                        }
                        0x2F => {
                            // 32bit type
                            offset += 2;
                            let int = file.read_u32::<LittleEndian>().expect("INVALID FILE DATA!");
                            let mut counter = 0;
                            offset += 4;
                            let mut byte_string: Vec<u8> = Vec::with_capacity(2usize.pow(32));
                            while counter < int {
                                byte_string.push(file.read_u8().expect("INVALID FILE DATA!"));
                                offset += 1;
                                counter += 1;
                            }
                            data.push(TypeString(to_string(byte_string)));
                        }
                        0x3F => {
                            // 64bit type
                            offset += 2;
                            let int = file.read_u64::<LittleEndian>().expect("INVALID FILE DATA!");
                            let mut counter = 0;
                            offset += 8;
                            let mut byte_string: Vec<u8> = Vec::with_capacity(2usize.pow(64));
                            while counter < int {
                                byte_string.push(file.read_u8().expect("INVALID FILE DATA!"));
                                offset += 1;
                                counter += 1;
                            }
                            data.push(TypeString(to_string(byte_string)));
                        }
                        0x4F => {
                            // 128bit type
                            offset += 2;
                            let int = file
                                .read_u128::<LittleEndian>()
                                .expect("INVALID FILE DATA!");
                            let mut counter = 0;
                            offset += 16;
                            let mut byte_string: Vec<u8> = Vec::with_capacity(2usize.pow(128));
                            while counter < int {
                                byte_string.push(file.read_u8().expect("INVALID FILE DATA!"));
                                offset += 1;
                                counter += 1;
                            }
                            data.push(TypeString(to_string(byte_string)));
                        }
                        _ => {}
                    }
                }
                JMPFD => {
                    instructions.push(0x0C);
                    offset += 1;
                    let int = file.read_u64::<LittleEndian>().expect("INVALID FILE DATA!");
                    data.push(UInteger(int));
                    offset += 8;
                }
                JMPFS => {
                    instructions.push(0x0D);
                    data.push(Null);
                    offset += 1;
                }
                PRTFS => {
                    instructions.push(0x0E);
                    data.push(Null);
                    offset += 1;
                }
                _ => {
                    instructions.push(buffer[0]);
                    data.push(Null);
                    offset += 1;
                }
            }

            // println!("{offset}");

            let mut crsr_minus = crsr;
            crsr_minus += 2;

            if filelength == crsr_minus {
                break;
            }

            counter += 1;
        }

        FileReader { instructions, data }
    }
}
