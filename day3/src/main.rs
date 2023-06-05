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
    let mut lines_iter = reader.lines();

    loop {
        let mut group_of_lines = Vec::new();

        for _ in 0..3 {
            if let Some(line) = lines_iter.next() {
                group_of_lines.push(line?);
            } else {
                break;
            }
        }

        if group_of_lines.is_empty() || group_of_lines.len() < 3 {
            break;
        }

        let mut character: Option<char> = None;
        for char1 in group_of_lines.get(0).unwrap().chars() {
            for char2 in group_of_lines.get(1).unwrap().chars() {
                for char3 in group_of_lines.get(2).unwrap().chars() {
                    match char1 == char2 && char2 == char3 {
                        true => {
                            character = Some(char1);
                            break;
                        }
                        false => (),
                    }
                }
            }
        }

        if character.is_some() {
            println!("Line Group = {:#?}", group_of_lines);
            println!("Matching Character = {}", character.unwrap());
            matches.push(character.unwrap());
        }
    }

    let mut total_sum = 0;
    for char in matches.as_slice() {
        total_sum += get_priority(*char);
    }
    println!("{}", total_sum);

    Ok(())
}

