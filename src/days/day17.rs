use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Result},
};

type Fragment = Vec<Vec<u8>>;

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

trait UTF8 {
    fn is_incomplete(&self, side: Side) -> bool;
    fn is_left_incomplete(&self) -> bool;
    fn is_right_incomplete(&self) -> bool;
}

impl UTF8 for &Vec<u8> {
    fn is_incomplete(&self, side: Side) -> bool {
        match side {
            Side::Left => self.is_left_incomplete(),
            Side::Right => self.is_right_incomplete(),
        }
    }

    fn is_left_incomplete(&self) -> bool {
        matches!(self.first(), Some(&byte) if matches_bit_pattern(byte, &[1, 0]))
    }

    fn is_right_incomplete(&self) -> bool {
        match self[..] {
            [.., byte, _, _] if matches_bit_pattern(byte, &[1, 1, 1, 1]) => true,
            [.., byte, _] if matches_bit_pattern(byte, &[1, 1, 1]) => true,
            [.., byte] if matches_bit_pattern(byte, &[1, 1]) => true,
            _ => false,
        }
    }
}

impl UTF8 for &Fragment {
    fn is_incomplete(&self, side: Side) -> bool {
        self.iter().any(|line| line.is_incomplete(side))
    }

    fn is_left_incomplete(&self) -> bool {
        self.iter().any(|line| line.is_left_incomplete())
    }

    fn is_right_incomplete(&self) -> bool {
        self.iter().any(|line| line.is_right_incomplete())
    }
}

struct TreasureMap<'a> {
    rows: HashMap<usize, Vec<(usize, &'a Vec<u8>)>>,
}

impl<'a> TreasureMap<'a> {
    fn new() -> Self {
        Self {
            rows: HashMap::new(),
        }
    }

    fn insert_fragment(&mut self, start_row: usize, start_col: usize, fragment: &'a Fragment) {
        for (dx, line) in fragment.iter().enumerate() {
            self.rows
                .entry(start_row + dx)
                .or_default()
                .push((start_col, line));
        }
    }

    fn decode(&mut self) -> Vec<String> {
        let mut decoded: Vec<_> = self.rows.iter().collect();
        decoded.sort_by_key(|(x, _)| *x);

        decoded
            .into_iter()
            .map(|(_, entries)| {
                let mut entries = entries.clone();
                entries.sort_by_key(|(y, _)| *y);
                entries.dedup_by_key(|(y, _)| *y);
                let bytes = entries
                    .into_iter()
                    .flat_map(|(_, line)| line.iter().copied())
                    .collect::<Vec<u8>>();
                String::from_utf8(bytes).unwrap()
            })
            .collect()
    }

    fn _print(&mut self) {
        for line in self.decode() {
            println!("{}", line);
        }
    }
}

fn matches_bit_pattern(byte: u8, pattern: &[u8]) -> bool {
    pattern.iter().enumerate().all(|(i, &bit)| {
        let mask = 1 << (7 - i);
        bit == ((byte & mask) >> (7 - i))
    })
}

fn pieces_fit(left_piece: Fragment, right_piece: Fragment) -> bool {
    left_piece.iter().zip(right_piece.iter()).all(|(l, r)| {
        if !l.is_right_incomplete() && !r.is_left_incomplete() {
            return true;
        }

        let num_extra = r
            .iter()
            .take_while(|&&b| matches_bit_pattern(b, &[1, 0]))
            .count();

        match l.as_slice() {
            [.., b, _, _] if matches_bit_pattern(*b, &[1, 1, 1, 1, 0]) => num_extra == 1,
            [.., b, _] if matches_bit_pattern(*b, &[1, 1, 1, 1, 0]) => num_extra == 2,
            [.., b] if matches_bit_pattern(*b, &[1, 1, 1, 1, 0]) => num_extra == 3,
            [.., b, _] if matches_bit_pattern(*b, &[1, 1, 1, 0]) => num_extra == 1,
            [.., b] if matches_bit_pattern(*b, &[1, 1, 1, 0]) => num_extra == 2,
            [.., b] if matches_bit_pattern(*b, &[1, 1, 0]) => num_extra == 1,
            _ => false,
        }
    })
}

fn get_opposite(side: Side) -> Side {
    match side {
        Side::Left => Side::Right,
        Side::Right => Side::Left,
    }
}

pub fn solve() -> Result<String> {
    let mut input = String::new();
    BufReader::new(File::open("./input/17.txt")?).read_to_string(&mut input)?;

    let fragments: Vec<Fragment> = input
        .trim()
        .split("\n\n")
        .map(|f| {
            f.lines()
                .map(|line| {
                    line.chars()
                        .collect::<Vec<char>>()
                        .chunks(2)
                        .map(|c| c.iter().collect::<String>())
                        .map_while(|c| u8::from_str_radix(&c, 16).ok())
                        .collect::<Vec<u8>>()
                })
                .collect()
        })
        .collect();

    let top_left = fragments
        .iter()
        .find(|frag| {
            frag.first()
                .map_or(false, |line| line.starts_with(&[0xe2, 0x95, 0x94]))
        })
        .unwrap();

    let mut treasure_map = TreasureMap::new();
    treasure_map.insert_fragment(0, 0, top_left);

    let mut queue = vec![((0, 0), top_left.clone(), Side::Right)];

    while let Some(((x, y), fixed_piece, side)) = queue.pop() {
        if let Some(matching_piece) = fragments.iter().find(|other_piece| match side {
            Side::Left => pieces_fit(other_piece.to_vec(), fixed_piece.to_vec()),
            Side::Right => pieces_fit(fixed_piece.to_vec(), other_piece.to_vec()),
        }) {
            let new_y = match side {
                Side::Left => y - matching_piece.first().unwrap().len(),
                Side::Right => y + matching_piece.first().unwrap().len(),
            };

            treasure_map.insert_fragment(x, new_y, matching_piece);

            if matching_piece.is_incomplete(side) {
                queue.push(((x, new_y), matching_piece.clone(), side));
            }

            match fixed_piece.len().cmp(&matching_piece.len()) {
                std::cmp::Ordering::Greater => {
                    let remaining = fixed_piece[matching_piece.len()..].to_vec();
                    if (&remaining).is_incomplete(side) {
                        queue.push(((x + matching_piece.len(), y), remaining, side));
                    }
                }
                std::cmp::Ordering::Less => {
                    let remaining = matching_piece[fixed_piece.len()..].to_vec();
                    if (&remaining).is_incomplete(get_opposite(side)) {
                        queue.push((
                            (x + fixed_piece.len(), new_y),
                            remaining,
                            get_opposite(side),
                        ));
                    }
                }
                std::cmp::Ordering::Equal => {}
            }
        }
    }

    let decoded_treasure_map = treasure_map.decode();

    let (x, y) = decoded_treasure_map
        .iter()
        .enumerate()
        .find_map(|(x, line)| line.chars().position(|c| c == '╳').map(|y| (x, y)))
        .unwrap();

    Ok(format!(
        "The product of the x and y coordinates is {:?}.",
        x * y
    ))
}
