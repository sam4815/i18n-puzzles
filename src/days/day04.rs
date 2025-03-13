use std::{
    fs::File,
    io::{BufReader, Read, Result},
};

use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

pub fn solve() -> Result<String> {
    let mut input = String::new();
    BufReader::new(File::open("./input/04.txt")?).read_to_string(&mut input)?;

    let travel_time = input.trim().split("\n\n").fold(0, |total, trip| {
        let times: Vec<DateTime<Tz>> = trip
            .split("\n")
            .map(|location| {
                let parts = location.split_whitespace().collect::<Vec<&str>>();

                let timezone: Tz = parts[1].parse().unwrap();

                let naive_time = NaiveDateTime::parse_from_str(
                    &(parts[2..].join(" ") + ":00"),
                    "%b %d, %Y, %H:%M:%S",
                )
                .unwrap();

                timezone.from_local_datetime(&naive_time).unwrap()
            })
            .collect();

        total + times[1].signed_duration_since(times[0]).num_minutes()
    });

    Ok(format!("The total travel time is {} minutes.", travel_time))
}
