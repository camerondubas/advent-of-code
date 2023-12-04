use std::collections::HashMap;

use itertools::Itertools;

use crate::runner::run;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

pub fn part_1(input: &String) -> u32 {
    input.lines().fold(0, |acc, line| {
        let (_, nums) = line.split_once(": ").unwrap();
        let (winning_nums, your_nums) = nums.split_once(" | ").unwrap();

        let winning_nums = winning_nums.split_whitespace().collect_vec();
        let match_count = your_nums
            .split_whitespace()
            .filter(|x| winning_nums.contains(x))
            .count();

        if match_count == 0 {
            return acc;
        }

        acc + 2_u32.pow(match_count as u32 - 1)
    })
}

pub fn part_2(input: &String) -> u32 {
    input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut cards, (idx, line)| {
            let (_, nums) = line.split_once(": ").unwrap();
            let id = idx + 1;
            let (winning_nums, your_nums) = nums.split_once(" | ").unwrap();

            let winning_nums = winning_nums.split_whitespace().collect_vec();
            let match_count = your_nums
                .split_whitespace()
                .filter(|x| winning_nums.contains(x))
                .count();

            let current_card_count = cards.get(&id).unwrap_or(&0) + 1;
            cards.insert(id, current_card_count);

            for card_copy in 0..match_count {
                let card_copy = id + card_copy + 1;

                let card = cards.get(&card_copy).unwrap_or(&0);
                cards.insert(card_copy, card + current_card_count);
            }
            cards
        })
        .values()
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2023", "04", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 13);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2023", "04", None);
        let output = part_1(&input);
        assert_eq!(output, 20829);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2023", "04", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 30);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2023", "04", None);
        let output = part_2(&input);
        assert_eq!(output, 12648035);
    }
}
