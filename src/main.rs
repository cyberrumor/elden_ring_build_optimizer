use std::io::prelude::*;
pub mod core;

#[derive(Debug)]
struct IgnoreKeyword {
    name: [u8; core::MAX_NAME_LENGTH],
    length: usize,
}

impl IgnoreKeyword {
    fn new() -> Self {
        Self {
            name: [0; core::MAX_NAME_LENGTH],
            length: 0,
        }
    }

    fn set_name(&mut self, value: &str) {
        let bytes = value.as_bytes();
        let length = bytes.len().min(core::MAX_NAME_LENGTH);
        self.name[..length].copy_from_slice(&bytes[..length]);
        self.length = length;
    }

    fn get_name(&self) -> &str {
        let Ok(res) = std::str::from_utf8(&self.name[..self.length]) else {
            return &"";
        };
        res
    }
}

fn main() {
    let keys: std::vec::Vec<String> = vec![
        "physical".to_string(),
        "strike".to_string(),
        "slash".to_string(),
        "pierce".to_string(),
        "magic".to_string(),
        "fire".to_string(),
        "lightning".to_string(),
        "holy".to_string(),
        "immunity".to_string(),
        "robustness".to_string(),
        "focus".to_string(),
        "vitality".to_string(),
        "poise".to_string(),
    ];

    // Get the name of the stat the user wants to optimize.
    let maximize_stat: usize;
    loop {
        let mut input = String::new();
        for (index, key) in keys.iter().enumerate() {
            println!("{index}: {key:#?}");
        }
        print!("Choose index of stat to maximize: ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get user input.");
        maximize_stat = if let Ok(num) = input.trim().parse() {
            num
        } else {
            println!("{input} is not an available stat");
            continue;
        };
        break;
    }

    // Get keywords that should be ignored.
    let mut ignore_keywords: Vec<IgnoreKeyword> = Vec::new();
    loop {
        let mut input = String::new();
        print!("Choose a keyword to ignore ([enter] to skip): ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get user input.");
        input = input.trim().to_lowercase();
        match input.len() {
            0 => break,
            _ => {
                let mut ignore_keyword = IgnoreKeyword::new();
                ignore_keyword.set_name(&input);
                ignore_keywords.push(ignore_keyword);
            }
        }
    }

    // Ignore unobtainable items.
    for i in [
        "grass hair ornament",
        "deathbed smalls",
        "millicent's",
        "brave's",
        "golden prosthetic",
        "ragged",
    ] {
        let mut ignore_keyword = IgnoreKeyword::new();
        ignore_keyword.set_name(i);
        ignore_keywords.push(ignore_keyword);
    }

    // Get the available weight.
    let weight_restriction: f32;
    loop {
        let mut input = String::new();
        print!("Choose max armor weight: ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get user input.");
        weight_restriction = if let Ok(num) = input.trim().parse() {
            num
        } else {
            println!("Couldn't parse input into a number. Try again!");
            continue;
        };
        break;
    }
    let weight_restriction: u16 = (weight_restriction * 10.0) as u16;

    // Get the pieces.
    let mut pieces: Vec<core::ArmorPiece> = core::get_pieces(maximize_stat);

    pieces = pieces
        .into_iter()
        .filter(|piece| piece.weight < weight_restriction)
        .filter(|piece| {
            ignore_keywords
                .iter()
                .all(|ignore_keyword| !piece.get_name().contains(ignore_keyword.get_name()))
        })
        .collect();

    let result = core::get_set(weight_restriction, pieces);
    println!("\n{}", result);
}
