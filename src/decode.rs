use std::fs::File;
use image::png;
use image::{GenericImage, ImageDecoder};
use image::{ColorType, Rgb, Pixel};
pub fn from_image(f: File) -> Vec<u8> {
    decode(read_bytes(f))
}

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

fn decode(dat: Vec<Vec<u8>>) -> Vec<u8> {
    let mut body = dat.into_iter().skip_while(|l| is_empty(&l));
    let mut header = body.next().expect("empty body");
    // FIXME chunks? but we only want the first 3
    let size = decode_size(&[header[0], header[1], header[2]]);
    vec![]
}

fn is_empty(line: &[u8]) -> bool {
    let &first = line.first().expect("empty line?");
    // want something like all :: (a -> Bool) -> [a] -> Bool
    (first == 255 || first == 0) && line.iter().all(|&c| c == first)
}

fn decode_size(pixel: &[u8; 3]) -> usize {
    2
}
