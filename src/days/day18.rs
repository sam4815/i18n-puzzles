use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn solve() -> Result<String> {
    let _lines = BufReader::new(File::open("./input/18.txt")?).lines();

    Ok(format!("Not implemented!"))
}
