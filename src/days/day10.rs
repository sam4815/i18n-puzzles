extern crate bcrypt;
extern crate unicode_normalization;

use bcrypt::verify;
use itertools::Itertools;
use unicode_normalization::UnicodeNormalization;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Result},
};

pub fn generate_permutations(str: &str) -> impl Iterator<Item = String> {
    str.nfc()
        .map(|ch| {
            let composed = ch.to_string();
            let decomposed = ch.nfd().collect::<String>();

            if decomposed == composed {
                vec![composed]
            } else {
                vec![composed, decomposed]
            }
        })
        .multi_cartesian_product()
        .map(|parts| parts.concat())
}

pub fn solve() -> Result<String> {
    let mut input = String::new();
    BufReader::new(File::open("./input/10.txt")?).read_to_string(&mut input)?;

    let (hashed, attempts) = input.trim().split_once("\n\n").unwrap();

    let hash_map: HashMap<_, _> = hashed
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(username, password)| (username.to_string(), password.to_string()))
        .collect();

    let mut matches: HashMap<String, String> = HashMap::new();

    let valid_attempts = attempts.lines().filter(|line| {
        let (username, attempt) = line.split_once(" ").unwrap();

        if let Some(password) = matches.get(username) {
            return password.nfc().eq(attempt.nfc());
        }

        for variant in generate_permutations(attempt) {
            if let Some(hash) = hash_map.get(username) {
                if verify(&variant, hash).unwrap_or(false) {
                    matches.insert(username.to_string(), attempt.to_string());
                    return true;
                }
            }
        }

        false
    });

    Ok(format!(
        "There are {} valid login attempts.",
        valid_attempts.count()
    ))
}
