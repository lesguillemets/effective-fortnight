extern crate image;

use std::fs::File;
use std::path::Path;
use std::io::Read;
// use image;

mod encode;
mod decode;

// use encode;

fn main() {
    println!("Hello, world!");
    let file = match File::open("./src/encode.rs") {
        Ok(f) => f,
        Err(r) => panic!("{}", r),
    };
    let mut bytes = file.bytes();
    let imgbuf = encode::to_image(&mut bytes, 30);
    let mut outf = &mut File::create(&Path::new("out.png")).expect("fileerror");
    image::ImageRgb8(imgbuf).save(outf, image::PNG);

    let f2 = File::open("out.png").expect("HI");
    for line in decode::read_bytes(f2) {
        for c in line {
            print!("{}", c as char);
        }
    }
}
