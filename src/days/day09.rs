use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
};

use chrono::NaiveDate;

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/09.txt")?).lines();

    let nine_eleven = NaiveDate::parse_from_str("11-09-01", "%d-%m-%Y").unwrap();
    let formats = ["%Y-%m-%d", "%Y-%d-%m", "%m-%d-%Y", "%d-%m-%Y"];

    let people = lines.map_while(Result::ok).fold(
        HashMap::<String, Vec<String>>::new(),
        |mut people, line| {
            let (date, names) = line.split_once(": ").unwrap();
            for name in names.split(", ") {
                people
                    .entry(name.to_string())
                    .or_default()
                    .push(date.to_string());
            }

            people
        },
    );

    let mut nine_eleven_names = people
        .iter()
        .filter(|(_, dates)| {
            let format = formats
                .iter()
                .find(|format| {
                    dates
                        .iter()
                        .all(|date| NaiveDate::parse_from_str(date, format).is_ok())
                })
                .unwrap();

            dates
                .iter()
                .any(|date| NaiveDate::parse_from_str(date, format).unwrap() == nine_eleven)
        })
        .map(|(name, _)| name.clone())
        .collect::<Vec<String>>();

    nine_eleven_names.sort();

    Ok(format!(
        "The names of the people who wrote about 9/11 are {}",
        nine_eleven_names.join(" ")
    ))
}
