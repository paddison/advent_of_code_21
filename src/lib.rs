pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;

use std::{fs::File, io::Read};

pub fn parse_lines(file_name: &str) -> Vec<String> {
    let mut buffer= String::new();
    let _ = File::open(file_name)
        .expect("File not found.")
        .read_to_string(&mut buffer)
        .expect("unable to read to string");
    
            
    buffer.split("\n").map(|line| line.to_string()).collect()
}