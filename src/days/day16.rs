use std::{
    collections::HashMap, fs::File, io::{BufReader, Read, Result}
};

use encoding_rs::IBM866;

struct Neighbours {
    up: Option<char>,
    down: Option<char>,
    left: Option<char>,
    right: Option<char>,
}

macro_rules! box_chars {
    (single, down) => {
        '│' | '┌' | '┐' | '╤' | '┬' | '┤' | '├' | '╞' | '╡' | '┼'
    };
    (single, up) => {
        '│' | '┘' | '└' | '╧' | '┴' | '┤' | '├' | '╞' | '╡' | '┼'
    };
    (single, left) => {
        '─' | '┘' | '┐' | '┴' | '┬' | '╨' | '╥' | '┤' | '╢' | '┼'
    };
    (single, right) => {
        '─' | '└' | '┌' | '┴' | '┬' | '╨' | '╥' | '├' | '╟' | '┼'
    };
    (double, down) => {
        '║' | '╔' | '╗' | '╥' | '╦' | '╣' | '╠' | '╟' | '╢' | '╬'
    };
    (double, up) => {
        '║' | '╝' | '╚' | '╨' | '╩' | '╣' | '╠' | '╟' | '╢' | '╬'
    };
    (double, left) => {
        '═' | '╝' | '╗' | '╩' | '╦' | '╧' | '╤' | '╡' | '╣' | '╬'
    };
    (double, right) => {
        '═' | '╚' | '╔' | '╩' | '╦' | '╧' | '╤' | '╞' | '╠' | '╬'
    };
}

fn find_stable(c: char, neighbours: &Neighbours) -> Option<char> {
    match c {
        '│' | '─' => match neighbours {
            Neighbours { up: Some(box_chars!(single, down)), .. } => Some('│'),
            Neighbours { down: Some(box_chars!(single, up)), .. } => Some('│'),
            Neighbours { left: Some(box_chars!(single, right)), .. } => Some('─'),
            Neighbours { right: Some(box_chars!(single, left)), .. } => Some('─'),

            Neighbours { up: Some(up), .. } if !matches!(up, box_chars!(single, down)) => Some('─'),
            Neighbours { down: Some(down), .. } if !matches!(down, box_chars!(single, up)) => Some('─'),
            Neighbours { left: Some(left), .. } if !matches!(left, box_chars!(single, right)) => Some('│'),
            Neighbours { right: Some(right), .. } if !matches!(right, box_chars!(single, left)) => Some('│'),

            _ => None,
        },
        '║' | '═' => match neighbours {
            Neighbours { up: Some(box_chars!(double, down)), .. } => Some('║'),
            Neighbours { down: Some(box_chars!(double, up)), .. } => Some('║'),
            Neighbours { left: Some(box_chars!(double, right)), .. } => Some('═'),
            Neighbours { right: Some(box_chars!(double, left)), .. } => Some('═'),

            Neighbours { up: Some(up), .. } if !matches!(up, box_chars!(double, down)) => Some('═'),
            Neighbours { down: Some(down), .. } if !matches!(down, box_chars!(double, up)) => Some('═'),
            Neighbours { left: Some(left), .. } if !matches!(left, box_chars!(double, right)) => Some('║'),
            Neighbours { right: Some(right), .. } if !matches!(right, box_chars!(double, left)) => Some('║'),

            _ => None,
        },
        '╨' | '╥' | '╞' | '╡' => match neighbours {
            Neighbours { up: Some(box_chars!(double, down)), .. } => Some('╨'),
            Neighbours { down: Some(box_chars!(double, up)), .. } => Some('╥'),
            Neighbours { left: Some(box_chars!(double, right)), .. } => Some('╡'),
            Neighbours { right: Some(box_chars!(double, left)), .. } => Some('╞'),

            Neighbours { up: Some(box_chars!(single, down)), left: Some(left), .. } if !matches!(left, box_chars!(double, right)) => Some('╞'),
            Neighbours { up: Some(box_chars!(single, down)), right: Some(right), .. } if !matches!(right, box_chars!(double, left)) => Some('╡'),
            Neighbours { down: Some(box_chars!(single, up)), left: Some(left), .. } if !matches!(left, box_chars!(double, right)) => Some('╞'),
            Neighbours { down: Some(box_chars!(single, up)), right: Some(right), .. } if !matches!(right, box_chars!(double, left)) => Some('╡'),

            _ => None,
        },
        '╧' | '╤' | '╟' | '╢' => match neighbours {
            Neighbours { up: Some(box_chars!(single, down)), .. } => Some('╧'),
            Neighbours { down: Some(box_chars!(single, up)), .. } => Some('╤'),
            Neighbours { left: Some(box_chars!(single, right)), .. } => Some('╢'),
            Neighbours { right: Some(box_chars!(single, left)), .. } => Some('╟'),

            Neighbours { up: Some(box_chars!(double, down)), left: Some(left), .. } if !matches!(left, box_chars!(single, right)) => Some('╟'),
            Neighbours { up: Some(box_chars!(double, down)), right: Some(right), .. } if !matches!(right, box_chars!(single, left)) => Some('╢'),
            Neighbours { down: Some(box_chars!(double, up)), left: Some(left), .. } if !matches!(left, box_chars!(single, right)) => Some('╟'),
            Neighbours { down: Some(box_chars!(double, up)), right: Some(right), .. } if !matches!(right, box_chars!(single, left)) => Some('╢'),

            _ => None,
        },
        '╔' | '╗' | '╝' | '╚' => match neighbours {
            Neighbours { up: Some(box_chars!(double, down)), left: Some(box_chars!(double, right)), .. } => Some('╝'),
            Neighbours { up: Some(box_chars!(double, down)), right: Some(right), .. } if !matches!(right, box_chars!(double, left)) => Some('╝'),

            Neighbours { up: Some(box_chars!(double, down)), right: Some(box_chars!(double, left)), .. } => Some('╚'),
            Neighbours { up: Some(box_chars!(double, down)), left: Some(left), .. } if !matches!(left, box_chars!(double, right)) => Some('╚'),

            Neighbours { right: Some(box_chars!(double, left)), up: Some(up), .. } if !matches!(up, box_chars!(double, down)) => Some('╔'),
            Neighbours { right: Some(box_chars!(double, left)), down: Some(down), .. } if !matches!(down, box_chars!(double, up)) => Some('╚'),

            Neighbours { down: Some(box_chars!(double, up)), left: Some(box_chars!(double, right)), .. } => Some('╗'),
            Neighbours { down: Some(box_chars!(double, up)), right: Some(right), .. } if !matches!(right, box_chars!(double, left)) => Some('╗'),

            Neighbours { down: Some(box_chars!(double, up)), right: Some(box_chars!(double, left)), .. } => Some('╔'),
            Neighbours { down: Some(box_chars!(double, up)), left: Some(left), .. } if !matches!(left, box_chars!(double, right)) => Some('╔'),

            Neighbours { left: Some(box_chars!(double, right)), up: Some(up), .. } if !matches!(up, box_chars!(double, down)) => Some('╗'),
            Neighbours { left: Some(box_chars!(double, right)), down: Some(down), .. } if !matches!(down, box_chars!(double, up)) => Some('╝'),

            _ => None,
        },
        '┌' | '┐' | '┘' | '└' => match neighbours {
            Neighbours { up: Some(box_chars!(single, down)), left: Some(box_chars!(single, right)), .. } => Some('┘'),
            Neighbours { up: Some(box_chars!(single, down)), right: Some(right), .. } if !matches!(right, box_chars!(single, left)) => Some('┘'),

            Neighbours { up: Some(box_chars!(single, down)), right: Some(box_chars!(single, left)), .. } => Some('└'),
            Neighbours { up: Some(box_chars!(single, down)), left: Some(left), .. } if !matches!(left, box_chars!(single, right)) => Some('└'),

            Neighbours { right: Some(box_chars!(single, left)), up: Some(up), .. } if !matches!(up, box_chars!(single, down)) => Some('┌'),
            Neighbours { right: Some(box_chars!(single, left)), down: Some(down), .. } if !matches!(down, box_chars!(single, up)) => Some('└'),

            Neighbours { down: Some(box_chars!(single, up)), left: Some(box_chars!(single, right)), .. } => Some('┐'),
            Neighbours { down: Some(box_chars!(single, up)), right: Some(right), .. } if !matches!(right, box_chars!(single, left)) => Some('┐'),

            Neighbours { down: Some(box_chars!(single, up)), right: Some(box_chars!(single, left)), .. } => Some('┌'),
            Neighbours { down: Some(box_chars!(single, up)), left: Some(left), .. } if !matches!(left, box_chars!(single, right)) => Some('┌'),

            Neighbours { left: Some(box_chars!(single, right)), up: Some(up), .. } if !matches!(up, box_chars!(single, down)) => Some('┐'),
            Neighbours { left: Some(box_chars!(single, right)), down: Some(down), .. } if !matches!(down, box_chars!(single, up)) => Some('┘'),

            _ => None,
        },
        '┤' | '┴' | '├' | '┬' => match neighbours {
            Neighbours { up: Some(up), .. } if !matches!(up, box_chars!(single, down)) => Some('┬'),
            Neighbours { down: Some(down), .. } if !matches!(down, box_chars!(single, up)) => Some('┴'),
            Neighbours { left: Some(left), .. } if !matches!(left, box_chars!(single, right)) => Some('├'),
            Neighbours { right: Some(right), .. } if !matches!(right, box_chars!(single, left)) => Some('┤'),

            _ => None,
        },
        '╣' | '╩' | '╠' | '╦' => match neighbours {
            Neighbours { up: Some(up), .. } if !matches!(up, box_chars!(double, down)) => Some('╦'),
            Neighbours { down: Some(down), .. } if !matches!(down, box_chars!(double, up)) => Some('╩'),
            Neighbours { left: Some(left), .. } if !matches!(left, box_chars!(double, right)) => Some('╠'),
            Neighbours { right: Some(right), .. } if !matches!(right, box_chars!(double, left)) => Some('╣'),

            _ => None,
        },
        _ => None,
    }
}

fn count_rotations(start: char, end: char) -> i32 {
    let rotation_groups: Vec<(&[char], i32)> = vec![
        (&['│', '─'], 2),
        (&['║', '═'], 2),
        (&['╨', '╞', '╥', '╡'], 4),
        (&['╧', '╟', '╤', '╢'], 4),
        (&['╔', '╗', '╝', '╚'], 4),
        (&['┌', '┐', '┘', '└'], 4),
        (&['┤', '┴', '├', '┬'], 4),
        (&['╣', '╩', '╠', '╦'], 4),
    ];

    for (group, modulo) in rotation_groups {
        if let (Some(start_idx), Some(end_idx)) = (
            group.iter().position(|&c| c == start),
            group.iter().position(|&c| c == end),
        ) {
            return (end_idx as i32 - start_idx as i32).rem_euclid(modulo);
        }
    }

    0
}

fn get_cell(position: (usize, usize), cells: &HashMap<usize, HashMap<usize, char>>) -> Option<char> {
    match cells.get(&position.0) {
        Some(row) => row.get(&position.1).copied(),
        None => None,
    }
}

fn find_stable_neighbours(position: (usize, usize), stable_cells: &HashMap<usize, HashMap<usize, char>>) -> Neighbours {
    Neighbours {
        up: get_cell((position.0 - 1, position.1), stable_cells),
        down: get_cell((position.0 + 1, position.1), stable_cells),
        left: get_cell((position.0, position.1 - 1), stable_cells),
        right: get_cell((position.0, position.1 + 1), stable_cells),
    }
}

fn find_unstable_neighbours(position: (usize, usize), stable_cells: &HashMap<usize, HashMap<usize, char>>) -> Vec<(usize, usize)> {
    [
        (position.0 - 1, position.1),
        (position.0 + 1, position.1),
        (position.0, position.1 - 1),
        (position.0, position.1 + 1),
    ].iter().cloned().filter(|pos| get_cell(*pos, stable_cells).is_none()).collect()
}

fn _print_grid(num_rows: usize, num_cols: usize, stable_cells: &HashMap<usize, HashMap<usize, char>>) {
    for x in 0..num_rows {
        let mut chars = Vec::new();
        for y in 0..num_cols {
            if let Some(row) = stable_cells.get(&x) {
                chars.push(row.get(&y).unwrap_or(&' '));
            }
        }

        println!("{}", chars.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(""))
    }
}

pub fn solve() -> Result<String> {
    let mut input = Vec::new();
    BufReader::new(File::open("./input/16.txt")?).read_to_end(&mut input)?;

    let (string, _, _) = IBM866.decode(&input).to_owned();
    let lines: Vec<Vec<char>> = string.into_owned().lines().filter_map(|v| {
        if v.starts_with(" │") {
            Some(v.replace(" │ ║  ", "").replace("  ║ │░", "").to_string().chars().collect())
        } else {
            None
        }
    }).collect();

    let cells = lines.iter().enumerate().fold(HashMap::<usize, HashMap<usize, char>>::new(), |mut cells, (x, line)| {
        for (y, character) in line.iter().enumerate() {
            cells.entry(x).or_default().insert(y, *character);
        }

        cells
    });

    let mut stable_cells = lines.iter().enumerate().fold(HashMap::<usize, HashMap<usize, char>>::new(), |mut stable, (x, line)| {
        for (y, character) in line.iter().enumerate() {
            match *character {
                '│' | '─' | '┌' | '┐' | '┘' | '└' | '┤' | '┴' | '├' | '┬' | '╨' | '╥' | '╞' | '╡' | '╧' | '╤' | '╟' | '╢' | '║' | '═' | '╔' | '╗' | '╝' | '╚' | '╣' | '╩' | '╠' | '╦' => {},
                _ => {
                    stable.entry(x).or_default().insert(y, *character);
                },
            }
        }

        stable
    });

    stable_cells.entry(0).or_default().insert(1, '│');
    stable_cells.entry(lines.len() - 1).or_default().insert(lines[0].len() - 2, '│');

    let mut queue: Vec<(usize, usize)> = (0..lines.len())
        .flat_map(|x| (0..lines[0].len()).map(move |y| (x, y)))
        .collect();

    let mut rotations = 0;

    while let Some((x, y)) = queue.pop() {
        if get_cell((x, y), &stable_cells).is_none() {
            let neighbours = find_stable_neighbours((x, y), &stable_cells);

            if let Some(solution) = find_stable(get_cell((x, y), &cells).unwrap(), &neighbours) {
                stable_cells.entry(x).or_default().insert(y, solution);
                queue.splice(0..0, find_unstable_neighbours((x, y), &stable_cells));
                rotations += count_rotations(get_cell((x, y), &cells).unwrap(), solution);
            }
        }
    }

    _print_grid(lines.len(), lines[0].len(), &stable_cells);

    Ok(format!("The minimum number of required rotations is {}", rotations))
}
