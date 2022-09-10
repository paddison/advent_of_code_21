pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;

use std::{fs::File, io::Read};

pub fn parse_lines(file_name: &str) -> Vec<String> {
    let mut buffer = String::new();
    let _ = File::open(file_name)
        .expect("File not found.")
        .read_to_string(&mut buffer)
        .expect("unable to read to string");
            
    buffer.split('\n').map(|line| line.to_string()).collect()
}

// todo use generics for return values
pub fn split_commas(file_name: &str) -> Vec<u8> {
    let mut buffer = String::new();
    let _ = File::open(file_name)
        .expect("File not found.")
        .read_to_string(&mut buffer)
        .expect("unable to read to string");
    
        buffer.split(',').map(|n| n.parse::<u8>().expect("got non numeric input")).collect()
}

pub fn split_commas_32(file_name: &str) -> Vec<u32> {
    let mut buffer = String::new();
    let _ = File::open(file_name)
        .expect("File not found.")
        .read_to_string(&mut buffer)
        .expect("unable to read to string");
    
        buffer.split(',').map(|n| n.parse::<u32>().expect("got non numeric input")).collect()
}