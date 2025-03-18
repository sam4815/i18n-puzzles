use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
};

use unidecode::unidecode;

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/08.txt")?).lines();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let valid_passwords = lines.map_while(Result::ok).filter(|line| {
        let len = line.chars().count();
        let normalized = unidecode(&line.to_lowercase());

        (4..=12).contains(&len)
            && line.chars().any(|c| c.is_ascii_digit())
            && normalized.chars().any(|c| vowels.contains(&c))
            && normalized
                .chars()
                .any(|c| c.is_alphabetic() && !vowels.contains(&c))
            && normalized
                .chars()
                .fold(HashMap::new(), |mut map, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                })
                .iter()
                .all(|(_, v)| *v == 1)
    });

    Ok(format!(
        "There are {} valid passwords",
        valid_passwords.count()
    ))
}
