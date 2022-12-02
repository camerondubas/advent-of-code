use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("=== Part 1 ===");
    println!("Solution: {:?}", part_1(&contents));

    println!("=== Part 2 ===");
    println!("Solution: {:?}", part_2(&contents));
}

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSE: u32 = 0;

fn part_1(input: &String) -> u32 {
    let scores = input.lines().map(|line| {
        let pair: Vec<u32> = line
            .chars()
            .filter_map(|c| match c {
                'A' | 'X' => Some(ROCK),
                'B' | 'Y' => Some(PAPER),
                'C' | 'Z' => Some(SCISSORS),
                _ => None,
            })
            .collect();

        let score = match (pair[0], pair[1]) {
            (ROCK, PAPER) => WIN,
            (ROCK, SCISSORS) => LOSE,
            (PAPER, ROCK) => LOSE,
            (PAPER, SCISSORS) => WIN,
            (SCISSORS, PAPER) => LOSE,
            (SCISSORS, ROCK) => WIN,
            _ => DRAW,
        };

        score + pair[1]
    });

    scores.sum()
}

fn part_2(input: &String) -> u32 {
    let scores = input.lines().map(|line| {
        let pair: Vec<u32> = line
            .chars()
            .filter_map(|c| match c {
                'A' => Some(ROCK),
                'B' => Some(PAPER),
                'C' => Some(SCISSORS),
                'X' => Some(LOSE),
                'Y' => Some(DRAW),
                'Z' => Some(WIN),
                _ => None,
            })
            .collect();

        let score = match (pair[0], pair[1]) {
            (ROCK, WIN) => PAPER,
            (ROCK, LOSE) => SCISSORS,
            (PAPER, WIN) => SCISSORS,
            (PAPER, LOSE) => ROCK,
            (SCISSORS, WIN) => ROCK,
            (SCISSORS, LOSE) => PAPER,
            _ => pair[0],
        };

        score + pair[1]
    });

    scores.sum()
}
