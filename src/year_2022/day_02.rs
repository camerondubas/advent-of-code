use crate::runner::run;

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSE: u32 = 0;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

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
