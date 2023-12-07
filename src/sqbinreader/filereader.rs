use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::{fs, process};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::vminternals::immediates::Immediates;


#[derive(Clone)]
pub struct FileReader {
    pub instructions: Vec<u8>,
    pub data: Vec<Immediates>
}

impl FileReader {

    pub fn new(file_location:String) -> FileReader {
        let mut instructions:Vec<u8> = Vec::new();
        let mut data:Vec<Immediates> = Vec::new();
        let file = File::open(file_location.clone());
        let mut file = match file {
            Ok(file) => {
                file
            }
            Err(error) => {
                eprintln!("\x1B[31m{}\x1b[0m", error);
                process::exit(2);
            }
        };

        let mut offset = 0x00;
        let filelength = fs::metadata(file_location.clone()).expect("INVALID FILE METADATA").len();
        // println!("{filelength}");

        loop {

            let mut crsr = file.seek(SeekFrom::Start(offset)).unwrap();

            let mut buffer = [0u8, 0u8];

            match file.read_exact(&mut buffer){
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

            println!("Cursor: {}, Buffer: {:?}", crsr, buffer);
            //
            // println!("Idk");

            match buffer[0] {
                0x00 => {
                    // println!("Idk");
                    instructions.push(0x00);
                    data.push(Immediates::Null);
                    break;
                }
                0x01 => {
                    instructions.push(buffer[0]);
                    data.push(Immediates::Null);
                    offset += 1;
                }
                0x0A => {
                    instructions.push(buffer[0]);
                    match buffer[1] {
                        0x00 => {
                            data.push(Immediates::Null);
                            offset += 2;
                        }
                        0x01 => {
                            crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                            file.read_exact(&mut buffer).expect("INVALID FILE!");
                            data.push(Immediates::Boolean(if buffer[1] == 1 {
                                true
                            } else if buffer[1] == 0{
                                false
                            }else {
                                panic!("INVALID FILE DATA!");
                            }));
                            offset += 2;
                        }
                        0x02 => {
                            offset += 2;
                            crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                            data.push(Immediates::Integer(file.read_i64::<LittleEndian>().expect("INVALID FILE DATA!")));
                            offset += 8;
                        }
                        0x03 => {
                            offset += 2;
                            crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                            data.push(Immediates::UInteger(file.read_u64::<LittleEndian>().expect("INVALID FILE DATA!")));
                            offset += 8;
                        }
                        0x04 => {
                            offset += 2;
                            crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                            data.push(Immediates::Float(file.read_f64::<LittleEndian>().expect("INVALID FILE DATA!")));
                            offset += 8;
                        }
                        0x0F => {
                            offset += 2;
                            crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                            let int = file.read_u8().expect("INVALID FILE DATA!");
                            let mut counter = 0;
                            offset += 1;
                            let mut string:Vec<u8> = Vec::with_capacity(255);
                            while counter < int {
                                crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                                string.push(file.read_u8().expect("INVALID FILE DATA!"));
                                offset += 1;
                                counter += 1;
                            }
                            match String::from_utf8(string) {
                                Ok(string) => {
                                    data.push(Immediates::String(string));
                                }
                                Err(e) => {
                                    panic!("INVALID FILE DATA!");
                                }
                            }
                            // if let Some(Immediates::String(stringerson)) = data.pop() {
                            //     println!("{stringerson}");
                            //     println!("{offset}");
                            // }
                        }
                        0x1F => {
                            offset += 2;
                            crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                            let int = file.read_u16::<LittleEndian>().expect("INVALID FILE DATA!");
                            let mut counter = 0;
                            offset += 2;
                            let mut string:Vec<u8> = Vec::with_capacity(2usize.pow(16));
                            while counter < int {
                                crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                                string.push(file.read_u8().expect("INVALID FILE DATA!"));
                                offset += 1;
                                counter += 1;
                            }
                            match String::from_utf8(string) {
                                Ok(string) => {
                                    data.push(Immediates::String(string));
                                }
                                Err(e) => {
                                    panic!("INVALID FILE DATA!");
                                }
                            }
                        }
                        0x2F => {
                            offset += 2;
                            crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                            let int = file.read_u32::<LittleEndian>().expect("INVALID FILE DATA!");
                            let mut counter = 0;
                            offset += 4;
                            let mut string:Vec<u8> = Vec::with_capacity(2usize.pow(32));
                            while counter < int {
                                crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                                string.push(file.read_u8().expect("INVALID FILE DATA!"));
                                offset += 1;
                                counter += 1;
                            }
                            match String::from_utf8(string) {
                                Ok(string) => {
                                    data.push(Immediates::String(string));
                                }
                                Err(e) => {
                                    panic!("INVALID FILE DATA!");
                                }
                            }
                        }
                        0x3F => {
                            offset += 2;
                            crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                            let int = file.read_u64::<LittleEndian>().expect("INVALID FILE DATA!");
                            let mut counter = 0;
                            offset += 8;
                            let mut string:Vec<u8> = Vec::with_capacity(2usize.pow(64));
                            while counter < int {
                                crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                                string.push(file.read_u8().expect("INVALID FILE DATA!"));
                                offset += 1;
                                counter += 1;
                            }
                            match String::from_utf8(string) {
                                Ok(string) => {
                                    data.push(Immediates::String(string));
                                }
                                Err(e) => {
                                    panic!("INVALID FILE DATA!");
                                }
                            }
                        }
                        0x4F => {
                            offset += 2;
                            crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                            let int = file.read_u128::<LittleEndian>().expect("INVALID FILE DATA!");
                            let mut counter = 0;
                            offset += 16;
                            let mut string:Vec<u8> = Vec::with_capacity(2usize.pow(128));
                            while counter < int {
                                crsr = file.seek(SeekFrom::Start(offset)).unwrap();
                                string.push(file.read_u8().expect("INVALID FILE DATA!"));
                                offset += 1;
                                counter += 1;
                            }
                            match String::from_utf8(string) {
                                Ok(string) => {
                                    data.push(Immediates::String(string));
                                }
                                Err(e) => {
                                    panic!("INVALID FILE DATA!");
                                }
                            }
                        }
                        _ => {}
                    }
                }
                0x0E => {
                    instructions.push(0x0E);
                    data.push(Immediates::Null);
                    offset += 1;
                }
                _ => {

                }
            }

            // println!("{offset}");

        }

        FileReader {
            instructions,
            data
        }

    }

}