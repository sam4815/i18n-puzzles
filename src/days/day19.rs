use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Result},
};

use chrono::{LocalResult, NaiveDateTime, TimeZone};
use tzfile::Tz;

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/19.txt")?)
        .lines()
        .map_while(Result::ok);

    let times = lines
        .flat_map(|line| {
            let (naive_time_str, timezone_str) = line.split_once("; ").unwrap();
            let naive_time =
                NaiveDateTime::parse_from_str(naive_time_str, "%Y-%m-%d %H:%M:%S").unwrap();

            let mut results: Vec<String> = Vec::new();

            for version in &["2018c", "2018g", "2021b", "2023d"] {
                let path = format!("./input/zoneinfo/{}/{}", version, timezone_str);
                let source = std::fs::read(path).unwrap();

                let timezone = Tz::parse(timezone_str, &source).unwrap();

                if let LocalResult::Single(datetime) = (&timezone).from_local_datetime(&naive_time)
                {
                    results.push(datetime.to_utc().format("%+").to_string())
                }
            }

            results
        })
        .fold(HashMap::new(), |mut map, datetime| {
            *map.entry(datetime).or_insert(0) += 1;
            map
        });

    if let Some((time, _)) = times.iter().max_by_key(|(_, entry)| **entry) {
        return Ok(format!("The gravitational wave was recorded at {}", time));
    }

    Err(Error::new(ErrorKind::NotFound, "No solution found"))
}
