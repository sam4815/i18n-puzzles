extern crate bcrypt;
extern crate unicode_normalization;

use bcrypt::verify;
use itertools::Itertools;
use rayon::prelude::*;
use unicode_normalization::UnicodeNormalization;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Result},
};

pub fn get_permutations(str: &str) -> impl Iterator<Item = String> {
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

    let (accounts, attempts) = input.trim().split_once("\n\n").unwrap();

    let attempts: HashMap<String, Vec<&str>> = attempts
        .lines()
        .filter_map(|line| line.split_once(' '))
        .fold(HashMap::new(), |mut map, (username, attempt)| {
            map.entry(username.to_string()).or_default().push(attempt);
            map
        });

    let accounts: HashMap<_, _> = accounts
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(username, hash)| (hash.to_string(), attempts.get(username).unwrap().clone()))
        .collect();

    let valid_attempts = accounts.par_iter().flat_map(|(hash, attempts)| {
        let mut invalid: Vec<&str> = Vec::new();

        for attempt in attempts {
            if invalid.iter().any(|f| f.nfc().eq(attempt.nfc())) {
                continue;
            }

            if get_permutations(attempt).any(|variant| verify(&variant, hash).unwrap_or(false)) {
                return attempts
                    .iter()
                    .filter(|a| a.nfc().eq(attempt.nfc()))
                    .collect();
            } else {
                invalid.push(attempt);
            }
        }

        vec![]
    });

    Ok(format!(
        "There are {} valid login attempts.",
        valid_attempts.count()
    ))
}
