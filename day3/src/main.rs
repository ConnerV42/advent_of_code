use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader};

fn get_priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - ('a' as u32) + 1,
        'A'..='Z' => (c as u32) - ('A' as u32) + 27,
        _ => panic!("Invalid character"),
    }
}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut matches = Vec::new();
    for line in reader.lines() {
        let line = line?;

        let c1 = &line[0..line.len() / 2];
        let c2 = &line[line.len() / 2..line.len()];

        println!("1: {} 2: {}", c1, c2);

        let mut character = None;
        for char1 in c1.chars() {
            for char2 in c2.chars() {
                match char1 == char2 {
                    true => character = Some(char1),
                    false => ()
                }
            }
        }

        if character.is_some() {
            matches.push(character.unwrap());
        }
    }

    let mut total_sum = 0;
    for char in matches.as_slice() {
        total_sum += get_priority(*char);
    }
    println!("{}", total_sum);



    // now to organize priority?

    Ok(())
}

