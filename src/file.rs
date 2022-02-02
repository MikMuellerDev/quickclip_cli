use base64;
use std::{fs::File, io::{Read, Write}};
use inflate::inflate_bytes;
use deflate::deflate_bytes;


pub fn read_file(name: String) -> String {
    let mut file = File::open(&name).unwrap();
    let mut buf: Vec<u8> = vec![];
    file.read_to_end(&mut buf).unwrap();

    let compressed = deflate_bytes(&buf);

    println!("buf: {} compressed: {}", buf.len(), compressed.len());

    let content = base64::encode(compressed);
    return content;
}

pub fn write_file(string: String, filename: String) {
    let content = base64::decode(string).unwrap();
    
    let uncompressed = inflate_bytes(&content).unwrap();


    let mut file = File::create(filename).unwrap();

    // Write a slice of bytes to the file
    file.write_all(&uncompressed).unwrap();
}