use core::str;
use std::{
    fs::File,
    io::{BufReader, Read, Result},
};

use encoding_rs::mem::decode_latin1;

trait Latin {
    fn is_latin(&self) -> bool;
}

impl Latin for String {
    fn is_latin(&self) -> bool {
        self.chars().all(|c| {
            ('\u{0041}'..='\u{007A}').contains(&c) || // Basic Latin
                ('\u{00C0}'..='\u{00FF}').contains(&c) // Latin-1 Supplement
        })
    }
}

fn matches_clue(word: &str, clue: &str) -> bool {
    clue.chars().count() == word.chars().count()
        && clue
            .chars()
            .enumerate()
            .all(|(i, letter)| letter == '.' || letter == word.chars().nth(i).unwrap())
}

pub fn solve() -> Result<String> {
    let mut input = String::new();
    BufReader::new(File::open("./input/13.txt")?).read_to_string(&mut input)?;

    let (encoded_words, crossword) = input.trim().split_once("\n\n").unwrap_or(("", ""));

    let decoded_words: Vec<String> = encoded_words
        .lines()
        .map(|word| -> String {
            let u8_bytes: Vec<u8> = word
                .chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|c| c.iter().collect::<String>())
                .map_while(|c| u8::from_str_radix(&c, 16).ok())
                .collect::<Vec<u8>>();

            let u16_be_bytes: Vec<u16> = (0..u8_bytes.len() / 2)
                .map(|i| u16::from_be_bytes([u8_bytes[2 * i], u8_bytes[2 * i + 1]]))
                .collect();

            let u16_le_bytes: Vec<u16> = (0..u8_bytes.len() / 2)
                .map(|i| u16::from_le_bytes([u8_bytes[2 * i], u8_bytes[2 * i + 1]]))
                .collect();

            match u8_bytes[..] {
                [0xFE, 0xFF, ..] => String::from_utf16(&u16_be_bytes[1..]).unwrap(),
                [0xFF, 0xFE, ..] => String::from_utf16(&u16_le_bytes[1..]).unwrap(),
                [0xEF, 0xBB, 0xBF, ..] => String::from_utf8(u8_bytes[3..].into()).unwrap(),
                _ => {
                    match (
                        String::from_utf8(u8_bytes.clone()),
                        String::from_utf16(&u16_be_bytes),
                        String::from_utf16(&u16_le_bytes),
                    ) {
                        (Ok(word), _, _) if word.is_latin() => word,
                        (_, Ok(word), _) if word.is_latin() => word,
                        (_, _, Ok(word)) if word.is_latin() => word,
                        _ => decode_latin1(&u8_bytes).into_owned(),
                    }
                }
            }
        })
        .collect();

    let score = crossword.lines().fold(0, |score, clue| {
        match decoded_words
            .iter()
            .position(|word| matches_clue(word, clue.trim()))
        {
            Some(position) => score + position + 1,
            None => score,
        }
    });

    Ok(format!("The crossword solution is {}", score))
}
