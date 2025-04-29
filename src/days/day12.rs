use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

use unidecode::unidecode;

trait Sortable {
    fn normalize(&self) -> String;
    fn danish_normalize(&self) -> String;
    fn keep_alphanumeric(&self) -> String;
    fn strip_leading_lowercase(&self) -> String;
}

impl Sortable for str {
    fn normalize(&self) -> String {
        unidecode(self)
    }

    fn danish_normalize(&self) -> String {
        self.chars()
            .map(|c| match c {
                'Ä' | 'Ö' | 'Å' => c.to_string(),
                'Æ' => 'Ö'.to_string(),
                'Ø' => 'Ä'.to_string(),
                _ => unidecode(&c.to_string()),
            })
            .collect::<String>()
    }

    fn keep_alphanumeric(&self) -> String {
        self.chars().filter(|c| c.is_alphanumeric()).collect()
    }

    fn strip_leading_lowercase(&self) -> String {
        match self.find(|c: char| c.is_uppercase()) {
            Some(index) => self[index..].to_string(),
            None => "".to_string(),
        }
    }
}

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/12.txt")?).lines();
    let mut people: Vec<(String, i64)> = lines
        .map_while(Result::ok)
        .map(|line| {
            let (name, number) = line.split_once(": ").unwrap();
            (name.to_string(), number.parse::<i64>().unwrap())
        })
        .collect();

    people.sort_by_key(|(name, _)| name.normalize().keep_alphanumeric().to_lowercase());
    let (_, english_number) = people[people.len() / 2];

    people.sort_by_key(|(name, _)| name.danish_normalize().keep_alphanumeric().to_lowercase());
    let (_, danish_number) = people[people.len() / 2];

    people.sort_by_key(|(name, _)| {
        name.normalize()
            .strip_leading_lowercase()
            .keep_alphanumeric()
            .to_lowercase()
    });
    let (_, dutch_number) = people[people.len() / 2];

    Ok(format!(
        "The product of the three middle phone numbers is {}.",
        english_number * danish_number * dutch_number
    ))
}
