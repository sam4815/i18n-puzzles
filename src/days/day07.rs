use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

use chrono::{Duration, NaiveDateTime, TimeZone, Timelike};
use chrono_tz::Tz;

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/07.txt")?).lines();
    let halifax_timezone: Tz = "America/Halifax".parse().unwrap();
    let santiago_timezone: Tz = "America/Santiago".parse().unwrap();

    let fixed_times: Vec<NaiveDateTime> = lines
        .map_while(Result::ok)
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();

            let (naive_time_str, offset_str) = parts[0].split_once(".000").unwrap_or(("", ""));
            let naive_time =
                NaiveDateTime::parse_from_str(naive_time_str, "%Y-%m-%dT%H:%M:%S").unwrap();

            let halifax_time = halifax_timezone.from_local_datetime(&naive_time).unwrap();
            let santiago_time = santiago_timezone.from_local_datetime(&naive_time).unwrap();

            let mut time = if offset_str.starts_with(&santiago_time.offset().to_string()) {
                santiago_time
            } else {
                halifax_time
            };

            time -= Duration::minutes(parts[2].parse().unwrap());
            time += Duration::minutes(parts[1].parse().unwrap());

            time.naive_local()
        })
        .collect::<Vec<NaiveDateTime>>();

    let hour_sum = fixed_times
        .iter()
        .enumerate()
        .fold(0, |sum, (i, naive_time)| {
            sum + (i + 1) as u32 * naive_time.hour()
        });

    Ok(format!("The sum of the local hours is {}.", hour_sum))
}
