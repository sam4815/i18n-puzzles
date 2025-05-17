use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

#[derive(PartialEq)]
enum Direction {
    Ltr,
    Rtl,
}

fn reverse_brackets(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        c => c,
    }
}

fn strip_bidi(s: &str) -> String {
    s.replace("\u{2067}", "")
        .replace("\u{2066}", "")
        .replace("\u{2069}", "")
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

fn operate(operator: char, a: (i64, i64), b: (i64, i64)) -> (i64, i64) {
    match operator {
        '+' | '-' => {
            let lcm_den = lcm(a.1, b.1);
            let a_num = a.0 * (lcm_den / a.1);
            let b_num = b.0 * (lcm_den / b.1);
            match operator {
                '+' => (a_num + b_num, lcm_den),
                '-' => (a_num - b_num, lcm_den),
                _ => unreachable!(),
            }
        }
        '*' => (a.0 * b.0, a.1 * b.1),
        '/' => (a.0 * b.1, a.1 * b.0),
        _ => panic!("Unknown operator"),
    }
}

fn do_math(equation: &str, start_index: usize) -> ((i64, i64), usize) {
    let chars: Vec<char> = equation.chars().collect();

    let mut index = start_index;
    let mut operator = '+';
    let mut operand = (0, 1);
    let mut accumulator = (0, 1);

    while index < chars.len() {
        match chars[index] {
            '(' => (accumulator, index) = do_math(equation, index + 1),
            ')' => return (operate(operator, operand, accumulator), index),
            '+' | '-' | '/' | '*' => operator = chars[index],
            c if c.is_ascii_digit() => {
                accumulator = ((accumulator.0 * 10) + (c.to_digit(10).unwrap() as i64), 1)
            }
            ' ' if accumulator.0 != 0 => {
                operand = operate(operator, operand, accumulator);
                accumulator = (0, 1);
            }
            _ => {}
        }

        index += 1;
    }

    (operate(operator, operand, accumulator), index)
}

fn parse_equation(equation: &str, direction: Direction, start_index: usize) -> (String, usize) {
    let chars: Vec<char> = equation.chars().collect();

    let mut index = start_index;
    let mut parsed = String::new();

    while index < chars.len() {
        match chars[index] {
            '\u{2067}' if direction == Direction::Ltr => {
                let (result, next_index) = parse_equation(equation, Direction::Rtl, index + 1);
                index = next_index;
                parsed.push_str(&result);
            }
            '\u{2066}' if direction == Direction::Rtl => {
                let (result, next_index) = parse_equation(equation, Direction::Ltr, index + 1);
                index = next_index;
                parsed.push_str(&result);
            }
            '\u{2069}' => {
                return (
                    parsed
                        .chars()
                        .rev()
                        .map(reverse_brackets)
                        .collect::<String>(),
                    index,
                )
            }
            c if c.is_ascii_digit() && direction == Direction::Rtl => {
                let number = equation
                    .chars()
                    .skip(index)
                    .take_while(|d| d.is_ascii_digit())
                    .collect::<String>();
                parsed.push_str(&number.chars().rev().collect::<String>());
                index += number.len() - 1;
            }
            c => parsed.push(c),
        }

        index += 1;
    }

    (parsed, index)
}

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/18.txt")?).lines();

    let sum = lines.map_while(Result::ok).fold(0, |acc, line| {
        let (logical_num, logical_den) = do_math(&strip_bidi(&line), 0).0;
        let (visual_num, visual_den) = do_math(&parse_equation(&line, Direction::Ltr, 0).0, 0).0;

        acc + (logical_num / logical_den).abs_diff(visual_num / visual_den)
    });

    Ok(format!(
        "The sum total of differences between appearance and reality is {}.",
        sum
    ))
}
