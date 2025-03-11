use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/01.txt")?).lines();

    let sum = lines.fold(0, |total, line| {
        let line = line.unwrap();
        let char_length = line.chars().collect::<Vec<char>>().len();
        let byte_length = line.as_bytes().len();

        match (char_length <= 140, byte_length <= 160) {
            (false, false) => 0 + total,
            (true, false) => 7 + total,
            (false, true) => 11 + total,
            (true, true) => 13 + total,
        }
    });

    Ok(format!("The total cost of the messages is {} cents", sum))
}
