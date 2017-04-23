use std::fs::File;
use image::png;
use image::{GenericImage, ImageDecoder};
use image::{ColorType, Rgb, Pixel};
pub fn from_image() {}

pub fn read_bytes(f: File) -> Vec<Vec<u8>> {
    let mut decoder = png::PNGDecoder::new(f);
    let (w, h) = decoder.dimensions().expect("cannot get file dimensions");
    let row_len = decoder.row_len().expect("cannot get the row length");
    let mut v = Vec::new();
    for _ in 0..h {
        // TODO ; why do I have to provide the length?
        let mut row = vec![0; row_len];
        decoder.read_scanline(&mut row).expect("cannot read line");
        v.push(row);
    }
    v
}
