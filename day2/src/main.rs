use std::error::Error;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Calculation {
    ByMove,
    ByOutcome,
}
use Calculation::*;

// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}
use Outcome::*;

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("Invalid char: {}", c),
        }
    }
}

impl Outcome {
    fn get_score(self) -> u32 {
        match self {
            Lose => 0,
            Draw => 3,
            Win => 6,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
use Shape::*;

impl From<char> for Shape {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("Invalid char: {}", c),
        }
    }
}

impl Shape {
    fn get_score(self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn get_outcome(self, outcome: char) -> Shape {
        let outcome = Outcome::from(outcome);

        match self {
            Rock => match outcome {
                Win => Paper,
                Draw => Rock,
                Lose => Scissors,
            },
            Paper => match outcome {
                Win => Scissors,
                Draw => Paper,
                Lose => Rock,
            },
            Scissors => match outcome {
                Win => Rock,
                Draw => Scissors,
                Lose => Paper,
            },
        }
    }
}

// ----------------------------------------------------------------------------

fn get_outcome_score(them: Shape, you: Shape) -> u32 {
    match (them, you) {
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Lose,

        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,

        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Win,
    }
    .get_score()
}

fn get_round_score(them: char, you: char, score_type: Calculation) -> u32 {
    let them = Shape::from(them);
    let you = match score_type {
        ByMove => Shape::from(you),
        ByOutcome => them.get_outcome(you),
    };

    let shape_score = you.get_score();
    let outcome_score = get_outcome_score(them, you);

    shape_score + outcome_score
}

fn get_scores(lines: Vec<&str>, score_type: Calculation) -> Vec<u32> {
    lines
        .into_iter()
        .map(|line| {
            // This is trying to be too clever
            let [them, _, you]: [char; 3] = line.chars().collect::<Vec<char>>().try_into().unwrap();

            get_round_score(them, you, score_type)
        })
        .collect()
}

fn get_total(lines: &Vec<&str>, score_type: Calculation) -> u32 {
    get_scores(lines.clone(), score_type)
        .into_iter()
        .reduce(|accum, item| accum + item)
        .unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_str = fs::read_to_string("input.txt")?;
    let lines: Vec<&str> = file_str.lines().collect();

    let part1_total = get_total(&lines, ByMove);
    let part2_total = get_total(&lines, ByOutcome);

    println!("Part 1: Final score: {}", part1_total);
    println!("Part 2: Final score: {}", part2_total);

    Ok(())
}
