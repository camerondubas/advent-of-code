const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSE: u32 = 0;

pub fn part_1(input: &str) -> u32 {
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

pub fn part_2(input: &str) -> u32 {
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

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2022", "02", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 15);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2022", "02", None);
        let output = part_1(&input);
        assert_eq!(output, 9177);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2022", "02", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 12);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2022", "02", None);
        let output = part_2(&input);
        assert_eq!(output, 12111);
    }
}
