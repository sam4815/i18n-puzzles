use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Result},
};

use chrono::DateTime;

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/02.txt")?).lines();

    let times = lines
        .filter_map(|line| match line {
            Ok(line) => Some(DateTime::parse_from_str(&line, "%+")),
            Err(_) => None,
        })
        .filter_map(|dt| dt.ok())
        .fold(HashMap::new(), |mut map, datetime| {
            *map.entry(datetime).or_insert(0) += 1;
            map
        });

    let wave_time = times
        .iter()
        .find_map(|(k, v)| if *v == 4 { Some(k) } else { None });

    if let Some(time) = wave_time {
        return Ok(format!(
            "The gravitational wave was recorded at {}",
            time.to_utc().format("%+")
        ));
    }

    Err(Error::new(ErrorKind::NotFound, "No matching times found"))
}
