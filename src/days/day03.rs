use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/03.txt")?).lines();

    let valid_passwords = lines.map_while(Result::ok).filter(|line| {
        let len = line.chars().count();

        (4..=12).contains(&len)
            && line.chars().any(|c| c.is_ascii_digit())
            && line.chars().any(|c| c.is_uppercase())
            && line.chars().any(|c| c.is_lowercase())
            && line.chars().any(|c| (c as u32) > 0x7F)
    });

    Ok(format!(
        "There are {} valid passwords.",
        valid_passwords.count()
    ))
}
