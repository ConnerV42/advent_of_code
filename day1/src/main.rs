use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Elf {
    pub calories: usize, // calories will always be non-negative
}

impl Elf {
    pub fn new(calories: usize) -> Elf {
        Elf {
            calories
        }
    }
}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }

    let mut elves: Vec<Elf> = Vec::new();
    let mut calories = 0usize;

    for line in lines {
        if line.is_empty() {
            elves.push(Elf::new(calories));
            calories = 0;
        } else {
            calories += line.parse::<usize>().expect("Could not parse line to usize");
        }
    }

    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    println!("{:#?}", elves);

    Ok(())
}

