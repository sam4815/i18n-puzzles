use core::str;
use std::{
    fs::File,
    io::{BufReader, Read, Result},
};

use encoding_rs::mem::convert_utf8_to_latin1_lossy;

pub fn solve() -> Result<String> {
    let mut input = String::new();
    BufReader::new(File::open("./input/06.txt")?).read_to_string(&mut input)?;

    let (encoded_words, crossword) = input.split_once("\n\n").unwrap_or(("", ""));

    let decoded_words = encoded_words
        .split("\n")
        .enumerate()
        .map(|(i, word)| -> String {
            match ((i + 1) % 15, (i + 1) % 5, (i + 1) % 3) {
                (0, _, _) => convert_utf8_to_latin1(&convert_utf8_to_latin1(word)),
                (_, 0, _) | (_, _, 0) => convert_utf8_to_latin1(word),
                (_, _, _) => word.into(),
            }
        })
        .collect::<Vec<String>>();

    let score = crossword.split("\n").fold(0, |score, clue| {
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

fn convert_utf8_to_latin1(word: &str) -> String {
    let mut latin1_word = vec![0; word.len() * 2];
    let length = convert_utf8_to_latin1_lossy(word.as_bytes(), &mut latin1_word);

    str::from_utf8(&latin1_word[..length]).unwrap().to_string()
}

fn matches_clue(word: &str, clue: &str) -> bool {
    clue.chars().count() == word.chars().count()
        && clue
            .chars()
            .enumerate()
            .all(|(i, letter)| letter == '.' || letter == word.chars().nth(i).unwrap())
}
