use crate::vminternals::immediates::Immediates::{
    self, Boolean, Float, Integer, Null, String as TypeString, UInteger,
};
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::{fs, process};

#[derive(Clone)]
pub struct FileReader {
    pub instructions: Vec<u8>,
    pub data: Vec<Immediates>,
}

fn to_string(string: Vec<u8>) -> String {
    match String::from_utf8(string) {
        Ok(uf8string) => uf8string,
        Err(e) => {
            panic!("INVALID FILE DATA! {}", e);
        }
    }
}

impl FileReader {
    pub fn new(file_location: String) -> FileReader {
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
        let filelength = fs::metadata(file_location.clone())
            .expect("INVALID FILE METADATA")
            .len();
        // println!("{filelength}");

        loop {
            let crsr = file.seek(SeekFrom::Start(offset)).unwrap();

            let mut buffer = [0u8, 0u8];

            match file.read_exact(&mut buffer) {
                Ok(_) => {}
                Err(e) => {
                    // if e.to_string().contains("failed to fill whole buffer") {
                    //     buffer[0] = 0;
                    //     buffer[1] = 0;
                    // } else {
                    eprintln!("\x1B[31m{}\x1b[0m", e);
                    process::exit(2);
                    // }
                }
            };

            if counter > 1 && crsr == 0 {
                break;
            }

            println!(
                "Cursor: {}, Buffer: {:?}, Counter: {}",
                crsr, buffer, counter
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
                            file.read_exact(&mut buffer).expect("INVALID FILE!");
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

            let mut crsr_minus = crsr.clone();
            crsr_minus += 2;

            if filelength == crsr_minus {
                break;
            }

            counter += 1;
        }

        FileReader { instructions, data }
    }
}
