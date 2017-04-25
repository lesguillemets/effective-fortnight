extern crate image;

use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;
use std::env;
// use image;

mod encode;
mod decode;

// use encode;

fn main() {
    let from_file = env::args().nth(1).unwrap_or("./src/decode.rs".to_owned());
    encode_file(&from_file, "out.png");

    /// decode the encoded file to validate
    let test_file = from_file.clone() + ".redecoded";
    let mut test = File::create(&test_file).expect("er");
    test.write(&decode_file("out.png"));
}

fn encode_file(from_file: &str, to_file: &str) -> () {
    let file = File::open(from_file).expect("encode_file: cannot read file");
    let mut bytes = file.bytes();
    let imgbuf = encode::to_image(&mut bytes, 800);
    let mut outf = &mut File::create(&Path::new(to_file)).expect("fileerror");
    image::ImageRgb8(imgbuf).save(outf, image::PNG);
}

fn decode_file(from_file: &str) -> Vec<u8> {
    let f = File::open(from_file).expect("decode_file: cannot read");
    decode::from_image(f)
}
