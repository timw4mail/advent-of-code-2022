use std::error::Error;
use std::fs;

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
    fn get_score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

fn get_outcome_score(them: Shape, you: Shape) -> u32 {
    match (them, you) {
        // Loss
        (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => 0,

        // Tie
        (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,

        // Win
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 6,
    }
}

fn get_round_score(them: char, you: char) -> u32 {
    let them = Shape::from(them);
    let you = Shape::from(you);

    let shape_score = you.get_score();
    let outcome_score = get_outcome_score(them, you);

    shape_score + outcome_score
}

fn get_scores(lines: Vec<&str>) -> Vec<u32> {
    lines
        .into_iter()
        .map(|line| {
            // This is trying to be too clever
            let [them, _, you]: [char; 3] = line.chars().collect::<Vec<char>>().try_into().unwrap();

            get_round_score(them, you)
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_str = fs::read_to_string("input.txt")?;
    let lines: Vec<&str> = file_str.lines().collect();
    let scores = get_scores(lines);
    let total = scores
        .into_iter()
        .reduce(|accum, item| accum + item)
        .unwrap();

    println!("Part 1: Final score: {}", total);

    Ok(())
}
