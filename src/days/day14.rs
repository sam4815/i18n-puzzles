use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader, Result},
};

trait JapaneseNumber {
    fn as_decimal(&self) -> usize;
}

trait JapeneseUnit {
    fn as_metres(&self, unit: char) -> (usize, usize);
}

impl JapaneseNumber for str {
    fn as_decimal(&self) -> usize {
        let (total, group, digit) =
            self.chars()
                .fold((0, 0, 0), |(total, group, digit), c| match c {
                    '一' => (total, group, 1),
                    '二' => (total, group, 2),
                    '三' => (total, group, 3),
                    '四' => (total, group, 4),
                    '五' => (total, group, 5),
                    '六' => (total, group, 6),
                    '七' => (total, group, 7),
                    '八' => (total, group, 8),
                    '九' => (total, group, 9),
                    '十' => (total, group + (max(digit, 1) * 10), 0),
                    '百' => (total, group + (max(digit, 1) * 100), 0),
                    '千' => (total, group + (max(digit, 1) * 1000), 0),
                    '万' => (total + max(group + digit, 1) * 10_000, 0, 0),
                    '億' => (total + max(group + digit, 1) * 100_000_000, 0, 0),
                    _ => (total, group, digit),
                });

        total + group + digit
    }
}

impl JapeneseUnit for usize {
    fn as_metres(&self, unit: char) -> (usize, usize) {
        match unit {
            '間' => (self * 60, 33),
            '丈' => (self * 100, 33),
            '町' => (self * 3600, 33),
            '里' => (self * 129_600, 33),
            '毛' => (self * 10, 33 * 10_000),
            '厘' => (self * 10, 33 * 1000),
            '分' => (self * 10, 33 * 100),
            '寸' => (self * 10, 33 * 10),
            _ => (self * 10, 33),
        }
    }
}

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/14.txt")?).lines();

    let sum: usize = lines
        .map_while(Result::ok)
        .map(|line| {
            let (a, b) = line.split_once(" × ").unwrap();
            let (mut a_qty, mut b_qty) = (a.to_string(), b.to_string());
            let (a_unit, b_unit) = (a_qty.pop().unwrap(), b_qty.pop().unwrap());

            let (a_num, a_den) = a_qty.as_decimal().as_metres(a_unit);
            let (b_num, b_den) = b_qty.as_decimal().as_metres(b_unit);

            (a_num * b_num) / (a_den * b_den)
        })
        .sum();

    Ok(format!("The total area is {}m²", sum))
}
