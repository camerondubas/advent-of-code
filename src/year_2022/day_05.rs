use crate::runner::run;
use itertools::Itertools;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

fn compute_stacks(raw_stacks: &str) -> Vec<Vec<char>> {
    let mut rev = raw_stacks.lines().rev();

    // ignore the first line
    rev.next();

    let mut computed_stacks: Vec<Vec<char>> = vec![];
    for line in rev {
        line.chars().enumerate().for_each(|(char_idx, char)| {
            if char.is_alphabetic() {
                let stack_idx = (char_idx - 1) / 4;
                if computed_stacks.len() <= stack_idx {
                    computed_stacks.push(vec![]);
                }
                computed_stacks[stack_idx].push(char);
            }
        });
    }

    computed_stacks
}

fn compute_instructions(raw_instructions: &str) -> Vec<(usize, usize, usize)> {
    let mut computed_instructions: Vec<(usize, usize, usize)> = vec![];
    for instruction in raw_instructions.lines() {
        let computed_instruction = instruction
            .split_whitespace()
            .filter_map(|word| match word.parse::<usize>() {
                Err(_) => None,
                Ok(num) => Some(num),
            })
            .collect_tuple::<(usize, usize, usize)>()
            .unwrap();
        computed_instructions.push(computed_instruction);
    }

    computed_instructions
}
fn part_1(input: &String) -> String {
    let mut sections = input.split("\n\n");
    let mut computed_stacks = compute_stacks(sections.next().unwrap());
    let computed_instructions = compute_instructions(sections.next().unwrap());

    for (count, from, to) in computed_instructions.iter() {
        let split_index = computed_stacks[from - 1].len() - count;

        let mut items = computed_stacks[from - 1].split_off(split_index);
        items.reverse();
        computed_stacks[to - 1].append(&mut items);
    }
    computed_stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .join("")
}

fn part_2(input: &String) -> String {
    let mut sections = input.split("\n\n");
    let mut computed_stacks = compute_stacks(sections.next().unwrap());
    let computed_instructions = compute_instructions(sections.next().unwrap());

    for (count, from, to) in computed_instructions.iter() {
        let split_index = computed_stacks[from - 1].len() - count;

        let mut items = computed_stacks[from - 1].split_off(split_index);
        // Part 2 Change 👇
        // items.reverse();
        computed_stacks[to - 1].append(&mut items);
    }
    computed_stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .join("")
}
