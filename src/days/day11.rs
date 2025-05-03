use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Result},
};

pub fn rotate_greek_left(str: &str) -> String {
    str.chars()
        .map(|c| match c {
            'σ' => 'ρ',
            'Α' => 'Ω',
            'α' => 'ω',
            c if (c as u32) >= 0x0370 => char::from_u32((c as u32) - 1).unwrap(),
            _ => c,
        })
        .collect()
}

pub fn solve() -> Result<String> {
    let lines = BufReader::new(File::open("./input/11.txt")?).lines();
    let mut odysseus_map = HashMap::<String, usize>::new();

    for odysseus in ["Οδυσσευς", "Οδυσσεως", "Οδυσσει", "Οδυσσεα", "Οδυσσευ"]
    {
        let mut current = odysseus.to_string();
        for i in 1..25 {
            current = rotate_greek_left(&current);
            odysseus_map.insert(current.to_string(), i);
        }
    }

    let shifts: Vec<usize> = lines
        .map_while(Result::ok)
        .map(|l| {
            for (variant, i) in &odysseus_map {
                if l.replace('ς', "σ").contains(variant) {
                    return *i;
                }
            }

            0
        })
        .collect();

    Ok(format!(
        "The sum total of alphabet shifts is {}.",
        shifts.iter().sum::<usize>()
    ))
}
