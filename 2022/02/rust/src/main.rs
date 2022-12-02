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

const ROCK: u8 = 1;
const PAPER: u8 = 2;
const SCISSORS: u8 = 3;

const WIN: u8 = 6;
const DRAW: u8 = 3;
const LOSE: u8 = 0;

fn part_1(input: &String) -> u32 {
    let pairs = input.lines().map(|line| {
        line.chars()
            .filter_map(|c| match c {
                'A' | 'X' => Some(ROCK),
                'B' | 'Y' => Some(PAPER),
                'C' | 'Z' => Some(SCISSORS),
                _ => None,
            })
            .collect()
    });

    pairs.fold(0, |acc, pair: Vec<u8>| {
        let score = match (pair[0], pair[1]) {
            (ROCK, PAPER) => WIN,
            (ROCK, SCISSORS) => LOSE,
            (PAPER, ROCK) => LOSE,
            (PAPER, SCISSORS) => WIN,
            (SCISSORS, PAPER) => LOSE,
            (SCISSORS, ROCK) => WIN,
            _ => DRAW,
        };

        acc + Into::<u32>::into(score + pair[1])
    })
}

fn part_2(input: &String) -> u32 {
    let pairs = input.lines().map(|line| {
        line.chars()
            .filter_map(|c| match c {
                'A' => Some(ROCK),
                'B' => Some(PAPER),
                'C' => Some(SCISSORS),
                'X' => Some(LOSE),
                'Y' => Some(DRAW),
                'Z' => Some(WIN),
                _ => None,
            })
            .collect()
    });

    pairs.fold(0, |acc, pair: Vec<u8>| {
        let score = match (pair[0], pair[1]) {
            (ROCK, WIN) => PAPER,
            (ROCK, LOSE) => SCISSORS,
            (PAPER, WIN) => SCISSORS,
            (PAPER, LOSE) => ROCK,
            (SCISSORS, WIN) => ROCK,
            (SCISSORS, LOSE) => PAPER,
            _ => pair[0],
        };

        acc + Into::<u32>::into(score + pair[1])
    })
}
