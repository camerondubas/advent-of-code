use colored::Colorize;
use std::env;
use std::fs;
use std::time::Instant;

fn get_offset(is_uppercase: bool) -> u32 {
    if is_uppercase {
        64 - 26
    } else {
        96
    }
}

fn run(name: &str, func: fn(&String) -> u32, input: &String) -> () {
    println!("=== {} ===", format!("{}", name).bold());
    let start = Instant::now();
    let out = func(input);
    let duration = start.elapsed();
    println!("Solution: {}", format!("{}", out).bold().green());
    println!("Duration: {}", format!("{:?}", duration).bold().blue());
    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
    run("Part 2 Sorted", part_2_sorted, &contents);
}

fn part_1(input: &String) -> u32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .fold(0, |acc, (left, right)| {
            let duplicate = left
                .chars()
                .find(|lc| right.chars().any(|rc| rc.eq(lc)))
                .unwrap();
            acc + (duplicate as u32) - get_offset(duplicate.is_uppercase())
        })
}

fn part_2(input: &String) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    lines.chunks(3).fold(0, |acc, group| {
        let badge = group[0]
            .chars()
            .find(|c| group[1].chars().any(|c2| c2.eq(c)) && group[2].chars().any(|c2| c2.eq(c)))
            .unwrap();

        acc + (badge as u32) - get_offset(badge.is_uppercase())
    })
}

fn part_2_sorted(input: &String) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    lines.chunks(3).fold(0, |acc, group| {
        let mut clone = group.to_vec();
        clone.sort();
        let badge = clone[0]
            .chars()
            .find(|c| group[1].chars().any(|c2| c2.eq(c)) && group[2].chars().any(|c2| c2.eq(c)))
            .unwrap();

        acc + (badge as u32) - get_offset(badge.is_uppercase())
    })
}
