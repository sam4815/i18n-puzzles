extern crate bcrypt;
extern crate unicode_normalization;

use bcrypt::verify;
use unicode_normalization::UnicodeNormalization;

use std::{
    collections::HashMap, fs::File, io::{BufReader, Read, Result}
};

pub fn find_permutations(str: String) -> Vec<String> {
    let mut perms = vec!["".to_string()];

    for char in str.chars().rev() {
        let decomposition = char.nfd().collect::<String>();
        let mut guaranteed = perms.iter().map(|p| decomposition.clone() + p).collect::<Vec<String>>();

        if decomposition.len() > 1 {
            guaranteed.extend(perms.iter().map(|p| char.to_string() + p).collect::<Vec<String>>());
        }

        perms = guaranteed
    }

    perms
}

pub fn solve() -> Result<String> {
    let mut input = String::new();
    BufReader::new(File::open("./input/10.txt")?).read_to_string(&mut input)?;

    let (hashed, attempts) = input.trim().split_once("\n\n").unwrap();
    let hash_map: HashMap<String, String> = hashed.split("\n").fold(
        HashMap::<String, String>::new(),
        |mut map, line| {
            let (username, password) = line.split_once(" ").unwrap();
            map.insert(username.to_string(), password.to_string());
            map
        },
    );

    let valid_attempts = attempts.split("\n").filter(|line| {
            println!("Starting");
            let (username, attempt) = line.split_once(" ").unwrap();

            let permutations = find_permutations(attempt.to_string());
            permutations.iter().any(|p| verify(p, hash_map.get(username).unwrap()).unwrap())
        }
    );

    println!("{:?}", "È".nfd().collect::<String>());

    Ok(format!(
        "There are {} valid login attempts.",
        valid_attempts.count()
    ))
}
