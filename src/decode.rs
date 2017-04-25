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

#[derive (Debug)]
struct DecodeInfo {
    start: usize,
    channel_width: usize,
    size: usize,
    height: usize,
    rem: usize,
}

impl DecodeInfo {
    fn new(s: usize, w: usize, sz: usize) -> DecodeInfo {
        DecodeInfo {
            start: s,
            channel_width: w,
            size: sz,
            height: (sz + w - 1) / w,
            rem: sz % w,
        }
    }
}

fn decode(dat: Vec<Vec<u8>>) -> Vec<u8> {
    let mut body = dat.into_iter().skip_while(|l| is_empty(&l));
    let mut header = body.next().expect("empty body");
    let info = handle_header(&header);
    read_body(body, &info)
}

fn read_body<F>(mut body: F, info: &DecodeInfo) -> Vec<u8>
    where F: Iterator<Item = Vec<u8>> // TODO: we could first skip(info.start)
{
    let read_line =
        |l: Vec<u8>| -> Vec<u8> { l.into_iter().skip(info.start).take(info.channel_width).collect() };
    let mut result = Vec::new();
    for _ in 0..info.height - 1 {
        let mut line = read_line(body.next().expect("lines less than expected"));
        result.append(&mut line);
    }
    let mut last = body.next()
        .expect("lines lessthan expected")
        .into_iter()
        .skip(info.start)
        .take(info.rem)
        .collect();
    result.append(&mut last);
    result
}


fn handle_header(header: &[u8]) -> DecodeInfo {
    let mut x = header.chunks(3).enumerate().skip_while(|&(i, p)| is_empty(p));
    let top_left = x.next().expect("empty header?");
    let start = top_left.0;
    let size = decode_size(top_left.1);
    let rightmost = x.fold(0, |acc, (i, px)| if !is_empty(px) { i } else { acc });
    DecodeInfo::new(3 * start, (rightmost - start + 1) * 3, size)
}

fn is_empty(line: &[u8]) -> bool {
    let &first = line.first().expect("empty line?");
    // want something like all :: (a -> Bool) -> [a] -> Bool
    (first == 255 || first == 0) && line.iter().all(|&c| c == first)
}

// TODO : something like &[u8;3]
fn decode_size(pixel: &[u8]) -> usize {
    pixel.iter()
        .map(|&n| n as usize)
        .fold((1, 0), |(base, acc), byte| (base * 256, acc + byte * base))
        .1
}
