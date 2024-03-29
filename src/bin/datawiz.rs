use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use clap::builder::Str;

mod my_reader {
    use std::{
        fs::File,
        io::{self, prelude::*},
    };

    pub struct BufReader {
        reader: io::BufReader<File>,
    }

    impl BufReader {
        pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
            let file = File::open(path)?;
            let reader = io::BufReader::new(file);

            Ok(Self { reader })
        }

        pub fn read_line<'buf>(&mut self, buffer: &'buf mut String) -> Option<io::Result<&'buf mut String>> {
            buffer.clear();

            self.reader
                .read_line(buffer)
                .map(|u| if u == 0 { None } else { Some(buffer) })
                .transpose()
        }
    }
}

fn parse_line(line: String, vector:&mut Vec<Vec<String>>, num_cols: Option<usize>) -> bool {
    let mut curr:String = String::from("");
    let mut quotes:usize = 0;
    let mut index:usize = 0;

    for (_i, c) in line.chars().enumerate() {
        if c == '"' {
            quotes = (quotes + 1) % 2;
            curr.clear();
        }

        else if c == ',' {
            if quotes == 0 {
                if num_cols.is_none() {
                    vector.push(vec![curr.clone()]);
                    index += 1;
                }

                else if index < num_cols.unwrap() {
                    vector[index].push(curr.clone());
                    index += 1;

                }
                
                return false;
            }
            curr.clear();
        }
        else {
            curr.push(c);
        }
    }

    return true;
}

fn read_file(file_path: String) -> std::io::Result<()> {
    let mut columnar_data:Vec<String> = Vec::new();
    let mut reader = my_reader::BufReader::open(file_path)?;
    let mut buffer = String::new();

    while let Some(line) = reader.read_line(&mut buffer) {
        let curr_line = line?.trim();
        let parts = curr_line.split(",");
    }

    Ok(())
}

fn infer_data_types() {

}
fn main() {

}