extern crate image;

use std::fs::File;
use std::io::Read;
use std::io::Bytes;
use image::*;

mod encode;

// use encode;

fn main() {
    println!("Hello, world!");
    let mut file = match File::open("./src/main.rs") {
        Ok(f) => f,
        Err(r) => panic!("{}", r),
    };
    let mut bytes = file.bytes();
    println!("{:?}", encode::encode(&mut bytes, 100))
}
