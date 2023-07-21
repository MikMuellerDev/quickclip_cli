use crate::colors;
use deflate::deflate_bytes;
use inflate::inflate_bytes;
use std::{
    fs::File,
    io::{Read, Write}, process,
};

pub fn read_file(name: String) -> String {
    let mut file = File::open(name).unwrap();
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf).unwrap();

    let compressed = deflate_bytes(&buf);
    let size_uncompressed: u32 = buf.len() as u32 / 1024;
    let size_compressed: u32 = compressed.len() as u32 / 1024;

    eprintln!(
        "{}: Source: {}KB compressed: {}KB. Size reduction: {}%",
        colors::blue("Deflate"),
        size_uncompressed,
        size_compressed,
        (100.0 - (size_compressed as f32 / size_uncompressed as f32) * 100.0).round()
    );

    base64::encode(compressed)
}

pub fn write_file(string: String, filename: String) {
    let content = base64::decode(string).unwrap_or_else(|e| {
        eprintln!("Could not decode base 64, Error: {e}");
        process::exit(1);
    });

    let uncompressed = inflate_bytes(&content).unwrap();

    let size_compressed: u32 = content.len() as u32 / 1024;
    let size_uncompressed: u32 = uncompressed.len() as u32 / 1024;

    let mut file = File::create(&filename).unwrap();

    eprintln!(
        "{}: Source: {}KB Uncompressed: {}KB. Size increased: {}%",
        colors::blue("Inflate"),
        size_compressed,
        size_uncompressed,
        (100.0 - (size_compressed as f32 / size_uncompressed as f32) * 100.0).round()
    );

    // Write a slice of bytes to the file
    file.write_all(&uncompressed).unwrap();
    eprintln!(
        "Contents written to file to {}. Final size: {}MB",
        filename,
        size_uncompressed as f32 / 1024.0,
    );
}
