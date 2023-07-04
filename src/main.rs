use std::io::prelude::*;
pub mod core;

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
        "immunity (unsupported)".to_string(),
        "robustness (unsupported)".to_string(),
        "focus (unsupported)".to_string(),
        "vitality (unsupported)".to_string(),
        "poise".to_string(),
    ];

    // Get the name of the stat the user wants to optimize.
    let mut maximize_stat: usize;
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
        if vec![8, 9, 10, 11].contains(&maximize_stat) {
            println!("Can't pick immunity, robsutness, focus or vitality :c");
            continue;
        }
        break;
    }

    // Get keywords that should be ignored.
    let mut ignore_keywords: Vec<String> = Vec::new();
    loop {
        let mut input = String::new();
        print!("Choose a keyword to ignore ([enter] to skip): ");
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to get user input.");
        input = input.trim().to_lowercase().to_string();
        match input.len() {
            0 => break,
            _ => ignore_keywords.push(input),
        }
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
        .filter(|x| x.weight < weight_restriction)
        .collect();

    if !ignore_keywords.is_empty() {
        pieces = pieces
            .into_iter()
            .filter(|x| ignore_keywords.iter().all(|n| !x.name.contains(n)))
            .collect();
    }

    let result = core::get_set(weight_restriction, pieces);
    println!("\n{}", result);
}
