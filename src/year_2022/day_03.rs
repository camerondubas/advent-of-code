use crate::runner::run;

fn get_offset(is_uppercase: bool) -> u32 {
    if is_uppercase {
        64 - 26
    } else {
        96
    }
}

pub fn solution(contents: String) {
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
