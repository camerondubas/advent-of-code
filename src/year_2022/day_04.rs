use crate::runner::run;
use itertools::Itertools;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

fn parse_assignments(input: &String) -> Vec<(u32, u32, u32, u32)> {
    input
        .lines()
        .map(|assignment| {
            assignment
                .split(',')
                .flat_map(|range| range.split('-').collect::<Vec<&str>>())
                .map(|num| num.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32, u32, u32)>()
                .unwrap()
        })
        .collect()
}

fn part_1(input: &String) -> u32 {
    parse_assignments(input)
        .iter()
        .fold(0, |acc, (a1, a2, b1, b2)| {
            if ((a1..=a2).contains(&b1) && (a1..=a2).contains(&b2))
                || ((b1..=b2).contains(&a1) && (b1..=b2).contains(&a2))
            {
                acc + 1
            } else {
                acc
            }
        })
}

fn part_2(input: &String) -> u32 {
    parse_assignments(input)
        .iter()
        .fold(0, |acc, (a1, a2, b1, b2)| {
            if (*a1..=*a2).any(|a| (*b1..=*b2).contains(&a)) {
                acc + 1
            } else {
                acc
            }
        })
}
