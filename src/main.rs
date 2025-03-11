mod days;

use days::*;
use std::time::Instant;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    let days: Vec<u8> = match args.len() {
        n if n >= 2 => args[1..]
            .iter()
            .map(|x| {
                x.parse()
                    .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
            })
            .collect(),
        _ => (1..=20).step_by(1).collect(),
    };

    let mut runtime = 0.0;

    for day in days {
        let func = get_day_solver(day);

        let time = Instant::now();
        let solution = match func() {
            Ok(value) => value.to_string(),
            Err(e) => format!("Error running solution: {}", e),
        };
        let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

        println!("\n=== Day {:02} ===", day);
        println!(" · {}", solution);
        println!(" · Elapsed: {:.4} ms", elapsed_ms);

        runtime += elapsed_ms;
    }

    println!("Total runtime: {:.4} ms", runtime);
}

fn get_day_solver(day: u8) -> fn() -> io::Result<String> {
    match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        4 => day04::solve,
        5 => day05::solve,
        6 => day06::solve,
        7 => day07::solve,
        8 => day08::solve,
        9 => day09::solve,
        10 => day10::solve,
        11 => day11::solve,
        12 => day12::solve,
        13 => day13::solve,
        14 => day14::solve,
        15 => day15::solve,
        16 => day16::solve,
        17 => day17::solve,
        18 => day18::solve,
        19 => day19::solve,
        20 => day20::solve,
        _ => unimplemented!(),
    }
}
