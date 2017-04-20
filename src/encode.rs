use std::fs::File;
use std::io::Read;
use std::io::Bytes;

const EMPTY_PIXEL:u8  = 255;

pub fn encode(content: &mut Bytes<File>, width:usize) ->  Vec<Vec<[u8;3]>>{
    let pixels = to_rgbs(content);
    into_width(&pixels.as_slice(),width)
}

fn to_rgbs(content:&mut Bytes<File>) -> Vec<[u8;3]> {
    let mut pixels = Vec::new();
    let mut bytes = content.map(|e| e.ok());
    while let Some(Some(r)) = bytes.next() {
        // tedious
        // FIXME; take(self, usize) -> Take(self) .. not very handy here
        let g = joinM(bytes.next()).unwrap_or(EMPTY_PIXEL);
        let b = joinM(bytes.next()).unwrap_or(EMPTY_PIXEL);
        pixels.push([r,g,b]);
    }
    pixels
}

fn into_width<T>(v: &[T], width:usize) -> Vec<Vec<T>>
where T:Clone
{
    let mut c = 0;
    let mut result = Vec::new();
    let mut row = Vec::new();
    for i in v {
        if c < width {
            c += 1;
            row.push(i.clone());
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
fn joinM<T> (e:Option<Option<T>>) -> Option<T> {
    e.unwrap_or(None)
}
