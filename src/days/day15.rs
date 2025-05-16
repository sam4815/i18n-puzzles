use std::{
    fs::File,
    io::{BufReader, Read, Result},
};

use chrono::{
    DateTime, Datelike, NaiveDate, TimeZone, Timelike, Utc,
    Weekday::{Fri, Mon, Thu, Tue, Wed},
};
use chrono_tz::Tz;

trait Working {
    fn is_work_day(&self) -> bool;
    fn is_work_hour(&self) -> bool;
    fn is_holiday(&self, holidays: &[(u32, u32)]) -> bool;
}

impl Working for DateTime<Tz> {
    fn is_work_day(&self) -> bool {
        [Mon, Tue, Wed, Thu, Fri].contains(&self.weekday())
    }

    fn is_work_hour(&self) -> bool {
        (9..=16).contains(&self.hour()) || (self.hour() == 8 && self.minute() >= 30)
    }

    fn is_holiday(&self, holidays: &[(u32, u32)]) -> bool {
        holidays
            .iter()
            .any(|(day, month)| self.day() == *day && self.month() == *month)
    }
}

fn parse_line(line: &str) -> (Tz, Vec<(u32, u32)>) {
    let parts: Vec<&str> = line.splitn(3, '\t').collect();

    let timezone: Tz = parts[1].parse().unwrap();

    let holidays: Vec<(u32, u32)> = parts[2]
        .split(';')
        .map(|date_str| {
            let dt = NaiveDate::parse_from_str(date_str, "%d %B %Y").unwrap();
            (dt.day(), dt.month())
        })
        .collect();

    (timezone, holidays)
}

pub fn solve() -> Result<String> {
    let mut input = String::new();
    BufReader::new(File::open("./input/15.txt")?).read_to_string(&mut input)?;

    let (offices, customers) = input.trim().split_once("\n\n").unwrap();
    let offices: Vec<(Tz, Vec<(u32, u32)>)> = offices.lines().map(parse_line).collect();

    let start_timestamp = Utc
        .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
        .unwrap()
        .timestamp();

    let unstaffed_periods: Vec<DateTime<Utc>> = (0..(2 * 24 * 365))
        .filter_map(|i| {
            let timestamp = start_timestamp + (30 * 60 * i);
            let utc_datetime: DateTime<Utc> = DateTime::from_timestamp(timestamp, 0).unwrap();

            if offices.iter().any(|(timezone, holidays)| {
                let local = utc_datetime.with_timezone(timezone);
                local.is_work_day() && local.is_work_hour() && !local.is_holiday(holidays)
            }) {
                return None;
            }

            Some(utc_datetime)
        })
        .collect();

    let overtimes: Vec<usize> = customers
        .lines()
        .map(parse_line)
        .map(|(timezone, holidays)| {
            let overtime_periods = unstaffed_periods.iter().filter(|utc_datetime| {
                let local = utc_datetime.with_timezone(&timezone);
                local.is_work_day() && !local.is_holiday(&holidays)
            });

            overtime_periods.count() * 30
        })
        .collect();

    Ok(format!(
        "The difference between the highest and lowest amount of overtime per customer is {}",
        overtimes.iter().max().unwrap() - overtimes.iter().min().unwrap()
    ))
}
