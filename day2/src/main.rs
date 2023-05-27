use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader};

#[derive(PartialEq, Copy, Clone)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn from_char(input: char) -> Option<Rps> {
        match input {
            'A' => Some(Rps::Rock),
            'B' => Some(Rps::Paper),
            'C' => Some(Rps::Scissors),
            _ => None,
        }
    }

    fn how_to_win(&self) -> Rps {
        match self {
            Rps::Rock => Rps::Paper,
            Rps::Paper => Rps::Scissors,
            Rps::Scissors=> Rps::Rock,
        }
    }

    fn how_to_lose(&self) -> Rps {
        match self {
            Rps::Rock => Rps::Scissors,
            Rps::Paper => Rps::Rock,
            Rps::Scissors => Rps::Paper,
        }
    }
}

struct RpsGame {
    me: Rps,
    them: Rps
}

impl RpsGame {
    fn from_chars(me: char, them: char) -> Option<RpsGame> {
        let them = Rps::from_char(them)?;
        let me = match me {
            'X' => them.how_to_lose(),
            'Y' => them,
            'Z' => them.how_to_win(),
            _ => return None,
        };

        Some(RpsGame { me, them })
    }

    fn get_score(&self) -> u32 {
        let base_score = match &self.me {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        };

        if self.me == self.them {
            base_score + 3
        } else if (self.me == Rps::Rock && self.them == Rps::Scissors)
            || (self.me == Rps::Paper && self.them == Rps::Rock)
            || (self.me == Rps::Scissors && self.them == Rps::Paper)
        {
            base_score + 6
        } else {
            base_score
        } 
    }

}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut total_score = 0u32;
    for line in reader.lines() {
        let line = line?;

        if line.len() >= 3 {
            if let Some(game) = RpsGame::from_chars(line.chars().nth(2).unwrap(), line.chars().next().unwrap()) {
                total_score += game.get_score();
            }
        }
    }

    println!("total_score: {}", total_score);

    Ok(())
}

