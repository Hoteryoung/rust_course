#![allow(dead_code)]
use std::{fs, fs::File, io::Read};

fn read_from_file() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("src/hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_from_file2() -> Result<String, std::io::Error> {
    fs::read_to_string("src/hello.txt")
}

fn read_from_file3() -> Result<String, Box<dyn std::error::Error>> {
    let s = fs::read_to_string("src/hello.txt")?;
    Ok(s)
}

pub fn exception_demo() {
    let f = File::open("src/hello.txt").unwrap();
    // let f = File::open("Cargo.tom").expect("failed_to_open_file");
    println!("{:?}", f);
    println!("{}", read_from_file().unwrap());
    println!("{}", read_from_file2().unwrap());
    println!("{}", read_from_file3().unwrap());
}

