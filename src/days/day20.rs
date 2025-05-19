use base64::prelude::*;
use std::{
    fs::File,
    io::{BufReader, Read, Result as IoResult},
};

fn read_utf16le(bytes: &[u8]) -> Vec<u16> {
    bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect()
}

fn get_bytes(s: &str) -> Vec<u8> {
    s.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|c| c.iter().collect::<String>())
        .map_while(|c| u8::from_str_radix(&c, 16).ok())
        .collect::<Vec<u8>>()
}

// As described in page 4 of https://www.rfc-editor.org/rfc/rfc2279
fn parse_utf(first: u8, bytes: Vec<u8>) -> u32 {
    let mut code = (first & (0xFF >> (bytes.len() + 2))) as u32;

    for b in bytes {
        code = (code << 6) | (b & 0b0011_1111) as u32;
    }

    code
}

fn get_sequence_length(byte: u8) -> usize {
    match byte {
        b if b & 0b1111_1100 == 0b1111_1100 => 6,
        b if b & 0b1111_1000 == 0b1111_1000 => 5,
        b if b & 0b1111_0000 == 0b1111_0000 => 4,
        b if b & 0b1110_0000 == 0b1110_0000 => 3,
        b if b & 0b1100_0000 == 0b1100_0000 => 2,
        b if b & 0b1000_0000 == 0 => 1,
        _ => 0,
    }
}

pub fn solve() -> IoResult<String> {
    let mut input = String::new();
    BufReader::new(File::open("./input/20.txt")?).read_to_string(&mut input)?;

    let decoded = BASE64_STANDARD.decode(input.replace("\n", "")).unwrap();

    let utf16_bytes = read_utf16le(&decoded[2..]);
    let utf16_string = String::from_utf16(&utf16_bytes).unwrap();

    let code_points = utf16_string
        .chars()
        .map(|s| format!("{:05x}", s as u32))
        .collect::<Vec<String>>()
        .join("");
    let u8_bytes = get_bytes(&code_points);

    let mut unicode_bytes: Vec<u32> = Vec::new();
    let mut i = 0;

    while i < u8_bytes.len() {
        let byte = u8_bytes.get(i).unwrap();
        let length = get_sequence_length(*byte);
        unicode_bytes.push(parse_utf(*byte, u8_bytes[(i + 1)..(i + length)].to_vec()));
        i += length;
    }

    let formatted_bytes = unicode_bytes
        .iter()
        .map(|b| format!("{:07x}", b))
        .collect::<Vec<String>>()
        .join("");

    let utf8_string = String::from_utf8(get_bytes(&formatted_bytes)).unwrap();

    Ok(format!("The message decodes to {}!", utf8_string))
}
