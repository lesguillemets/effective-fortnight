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
    let test_file = from_file.clone() + ".redecoded";
    let file = match File::open(from_file) {
        Ok(f) => f,
        Err(r) => panic!("{}", r),
    };
    let mut bytes = file.bytes();
    let imgbuf = encode::to_image(&mut bytes, 800);
    let mut outf = &mut File::create(&Path::new("out.png")).expect("fileerror");
    image::ImageRgb8(imgbuf).save(outf, image::PNG);

    let f2 = File::open("out.png").expect("HI");
    let mut test = File::create(test_file).expect("er");
    test.write(&decode::from_image(f2));
}
