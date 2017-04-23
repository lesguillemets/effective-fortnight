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

struct DecodeInfo {
    start: usize,
    width: usize,
    size: usize,
}

impl DecodeInfo {
    fn new(s: usize, w: usize, sz: usize) -> DecodeInfo {
        DecodeInfo {
            start: s,
            width: w,
            size: sz,
        }
    }
}

fn decode(dat: Vec<Vec<u8>>) -> Vec<u8> {
    let mut body = dat.into_iter().skip_while(|l| is_empty(&l));
    let mut header = body.next().expect("empty body");
    let info = handle_header(&header);
    // FIXME chunks? but we only want the first 3
    read_body(body, info)
}

fn read_body<F>(mut body: F, info: DecodeInfo) -> Vec<u8>
    where F: Iterator,
          F::Item: u7
{
    vec![]
}


fn handle_header(header: &[u8]) -> DecodeInfo {
    let mut x = header.chunks(3).enumerate().skip_while(|&(i, p)| is_empty(p));
    let top_left = x.next().expect("empty header?");
    let start = top_left.0;
    let size = decode_size(top_left.1);
    let mut rightmost = 0;
    while let Some((i, px)) = x.next() {
        if !is_empty(px) {
            rightmost = i;
        }
    }
    DecodeInfo::new(start, rightmost - start + 1, size)
}

fn is_empty(line: &[u8]) -> bool {
    let &first = line.first().expect("empty line?");
    // want something like all :: (a -> Bool) -> [a] -> Bool
    (first == 255 || first == 0) && line.iter().all(|&c| c == first)
}

// TODO : something like &[u8;3]
fn decode_size(pixel: &[u8]) -> usize {
    2
}
