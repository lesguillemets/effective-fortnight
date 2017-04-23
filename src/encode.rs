use std::fs::File;
use std::io::Read;
use std::io::Bytes;
use std::iter::Iterator;
use image;

// TODO
const EMPTY_PIXEL: u8 = 255;

pub fn to_image(content: &mut Bytes<File>,
                width: u32)
                -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let bytes = collect_bytes(content);
    let total_channels = bytes.len();
    let height = {
        let channel_width = width as usize * 3;
        1 + ((total_channels + channel_width - 1) / channel_width)
    };
    // TODO : check the height, separate the chunk if necessary
    let mut buf = image::ImageBuffer::new(width, height as u32);
    create_first_line(&mut buf, width, total_channels);
    let mut rgbs = to_rgbs(bytes).into_iter();
    {
        let mut pixels = buf.pixels_mut();
        for _ in 0..width {
            pixels.next();
        }
        while let Some(p) = rgbs.next() {
            let pixel = pixels.next().expect("er");
            *pixel = p;
        }
    }
    buf
}

fn create_first_line(buf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
                     width: u32,
                     len: usize) {
    buf[(0, 0)] = encode_size(len);
}

fn encode_size(size: usize) -> image::Rgb<u8> {
    // we want unfoldr
    let mut v = Vec::new();
    let mut s = size;
    for _ in 0..3 {
        let (d, m) = (size / 256, size % 256);
        s = d;
        v.push(m as u8);
    }
    image::Rgb([v[0], v[1], v[2]])
}


fn collect_bytes(content: &mut Bytes<File>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut bytes = content.map(|e| e.ok());
    while let Some(Some(r)) = bytes.next() {
        result.push(r)
    }
    result
}
fn to_rgbs(content: Vec<u8>) -> Vec<image::Rgb<u8>> {
    let emp = EMPTY_PIXEL; // hmm. FIXME
    let mut pixels = Vec::new();
    let mut bytes = content.iter();
    while let Some(r) = bytes.next() {
        // tedious
        // FIXME; take(self, usize) -> Take(self) .. not very handy here
        let g = bytes.next().unwrap_or(&emp);
        let b = bytes.next().unwrap_or(&emp);
        pixels.push(image::Rgb([*r, *g, *b]));
    }
    pixels
}

fn into_width<T>(v: T, width: usize) -> Vec<Vec<T::Item>>
    where T: Iterator
{
    let mut c = 0;
    let mut result = Vec::new();
    let mut row = Vec::new();
    for i in v {
        if c < width {
            c += 1;
            row.push(i);
        } else {
            c = 0;
            result.push(row);
            row = Vec::new();
        }
    }
    result.push(row);
    result
}

// TODO : make more polymorphic
fn joinM<T>(e: Option<Option<T>>) -> Option<T> {
    e.unwrap_or(None)
}
// What
