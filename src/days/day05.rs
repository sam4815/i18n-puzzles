use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/05.txt")?).lines();

    let (poops, _) = lines
        .skip(1)
        .map_while(Result::ok)
        .fold((0, 0), |(poops, x_pos), line| {
            let next_pos = (x_pos + 2) % line.chars().count();

            match line.chars().nth(next_pos) {
                Some('💩') => (poops + 1, next_pos),
                _ => (poops, next_pos),
            }
        });

    Ok(format!("You step in poop {} times.", poops))
}
