use base64;
use deflate::deflate_bytes;
use inflate::inflate_bytes;
use std::{
    fs::File,
    io::{Read, Write},
};

pub fn read_file(name: String) -> String {
    let mut file = File::open(&name).unwrap();
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf).unwrap();

    let compressed = deflate_bytes(&buf);
    let size_uncompressed: u32 = buf.len() as u32 / 1024;
    let size_compressed: u32 = compressed.len() as u32 / 1024;

    println!(
        "[Deflate] Source: {}KB compressed: {}KB. Size reduction: {}%",
        size_uncompressed,
        size_compressed,
        (100.0 - (size_compressed as f32 / size_uncompressed as f32) * 100.0).round()
    );

    let content = base64::encode(compressed);
    return content;
}

pub fn write_file(string: String, filename: String) {
    let content = base64::decode(string).unwrap();

    let uncompressed = inflate_bytes(&content).unwrap();

    let size_compressed: u32 = content.len() as u32 / 1024;
    let size_uncompressed: u32 = uncompressed.len() as u32 / 1024;

    let mut file = File::create(filename).unwrap();

    println!(
        "[Inflate] Source: {}KB Uncompressed: {}KB. Size increased: {}%",
        size_compressed,
        size_uncompressed,
        (100.0 - (size_compressed as f32 / size_uncompressed as f32) * 100.0).round()
    );

    // Write a slice of bytes to the file
    file.write_all(&uncompressed).unwrap();
}
