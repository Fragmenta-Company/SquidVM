use crate::vminternals::immediates::Immediates::{
    self, Boolean, Float, Integer, Null, String as TypeString, UInteger,
};
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::{fs, process};

const SQDBIN: &str = ".sqdbin";

/// Holds all the instructions and data that
/// the VM will use in order to function properly.
#[derive(Clone)]
pub struct FileReader {
    pub instructions: Vec<u8>,
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
    pub fn new(mut file_location: String, filearg:bool) -> FileReader {
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
                process::exit(2);
            }
        };

        let mut offset = 0x00;
        let mut counter = 0;
        let filelength = fs::metadata(file_location)
            .expect("INVALID FILE METADATA")
            .len();
        // println!("{filelength}");

        loop {
            let mut crsr = file.seek(SeekFrom::Start(offset)).unwrap();

            let mut buffer = [0u8, 0u8];

            match file.read_exact(&mut buffer) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("\x1B[31m{}\x1b[0m", e);
                    process::exit(2);
                }
            };

            if filearg {

                match buffer[0] {
                    1 => {

                        offset += 1;

                        crsr = file.seek(SeekFrom::Start(offset)).unwrap();

                        let major = match file.read_u32::<LittleEndian>() {
                            Ok(byte) => {
                                byte
                            }
                            Err(err) => {
                                dev_print!("Error: {:?}", err);
                                eprintln!("\x1B[31mINVALID FILE METADATA!\x1b[0m");
                                process::exit(2);
                            }
                        };

                        offset+=4;

                        let minor = match file.read_u16::<LittleEndian>() {
                            Ok(byte) => {
                                byte
                            }
                            Err(err) => {
                                dev_print!("Error: {:?}", err);
                                eprintln!("\x1B[31mINVALID FILE METADATA!\x1b[0m");
                                process::exit(2);
                            }
                        };

                        offset+=2;

                        let patch = if let Ok(bytes) = file.read_u16::<LittleEndian>() {
                            bytes
                        } else if let Err(err) = file.read_u16::<LittleEndian>() {
                                dev_print!("Error: {:?}", err);
                                eprintln!("\x1B[31mINVALID FILE METADATA!\x1b[0m");
                                process::exit(2);
                        } else {
                            unreachable!()
                        };

                        offset+=2;

                        let details = if let Ok(byte) = file.read_u8() {
                            byte
                        } else if let Err(err) = file.read_u8() {
                            dev_print!("Error: {:?}", err);
                            eprintln!("\x1B[31mINVALID FILE METADATA!\x1b[0m");
                            process::exit(2);
                        } else {
                            unreachable!()
                        };

                        let mut counter = 0;
                        let mut byte_string = vec![0; 6];

                        while counter < 22 {
                            byte_string.push(match file.read_u8() {
                                Ok(int) => {
                                    int
                                }
                                Err(err) => {
                                    dev_print!("Error: {:?}", err);
                                    eprintln!("\x1B[31mINVALID FILE METADATA!\x1b[0m");
                                    process::exit(2);
                                }
                            });
                            offset += 1;
                            counter += 1;
                        }

                        offset += 1;

                        let details = match details {
                            0 => {
                                "Release"
                            }
                            1 => {
                                "Alpha"
                            }
                            2 => {
                                "Beta"
                            }
                            _ => {
                                "Unknown"
                            }
                        };

                        let version = format!("Compatible with {}.{}.{}-{} and up until next major",
                        major, minor, patch, details);

                        println!("{version}");
                        println!("\x1B[31mIn alpha and beta versions the VM will change a lot, so most things will change.");
                        println!("Binaries for SquidVM 0.7.0-alpha and up will not be compatible with old versions\x1b[0m");

                        let compiler = match String::from_utf8(byte_string) {
                            Ok(string) => {
                                string
                            }
                            Err(err) => {
                                dev_print!("Error: {:?}", err);
                                eprintln!("\x1B[31mINVALID FILE METADATA!\x1b[0m");
                                process::exit(2);
                            }
                        };

                        println!("Compiled with: {compiler}");

                        file.read_exact(&mut buffer).expect("INVALID FILE DATA!");

                    }
                    _ => {
                        println!("File doesn't include metadata!");
                    }
                }

                process::exit(0);

            }

            if counter < 1 && buffer[0] == 1 {

                offset+=32;

                crsr = file.seek(SeekFrom::Start(offset)).unwrap();

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
                0x00 => {
                    // println!("Idk");
                    instructions.push(0x00);
                    data.push(Null);
                    break;
                }
                0x01 => {
                    instructions.push(0x01);
                    data.push(Null);
                    offset += 1;
                }
                0x02 => {
                    instructions.push(0x02);
                    data.push(Null);
                    offset += 1;
                }
                0x03 => {
                    instructions.push(0x03);
                    data.push(Null);
                    offset += 1;
                }
                0x04 => {
                    instructions.push(0x04);
                    data.push(Null);
                    offset += 1;
                }
                0x05 => {
                    instructions.push(0x05);
                    data.push(Null);
                    offset += 1;
                }
                0x06 => {
                    instructions.push(0x06);
                    data.push(Null);
                    offset += 1;
                }
                0x07 => {
                    instructions.push(0x07);
                    data.push(Null);
                    offset += 1;
                }
                0x08 => {
                    instructions.push(0x08);
                    data.push(Null);
                    offset += 1;
                }
                0x09 => {
                    instructions.push(0x09);
                    data.push(Null);
                    offset += 1;
                }
                0x0A => {
                    instructions.push(buffer[0]);
                    match buffer[1] {
                        0x00 => {
                            data.push(Null);
                            offset += 2;
                        }
                        0x01 => {
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
                            offset += 2;
                            data.push(Integer(
                                file.read_i64::<LittleEndian>().expect("INVALID FILE DATA!"),
                            ));
                            offset += 8;
                        }
                        0x03 => {
                            offset += 2;
                            data.push(UInteger(
                                file.read_u64::<LittleEndian>().expect("INVALID FILE DATA!"),
                            ));
                            offset += 8;
                        }
                        0x04 => {
                            offset += 2;
                            data.push(Float(
                                file.read_f64::<LittleEndian>().expect("INVALID FILE DATA!"),
                            ));
                            offset += 8;
                        }
                        0x0F => {
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
                0x0C => {
                    instructions.push(0x0C);
                    offset += 1;
                    let int = file.read_u64::<LittleEndian>().expect("INVALID FILE DATA!");
                    data.push(UInteger(int));
                    offset += 8;
                }
                0x0E => {
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
