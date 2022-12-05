use crate::runner::run;
use itertools::Itertools;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

fn part_1(input: &String) -> String {
    let mut sections = input.split("\n\n");
    let mut stacks = sections.next().unwrap().lines().rev();
    let mut instructions = sections.next().unwrap();

    // ignore the first line
    stacks.next();

    let mut computed_stacks: Vec<Vec<char>> = vec![];
    for line in stacks {
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

    let mut computed_instructions: Vec<(u32, u32, u32)> = vec![];
    for instruction in instructions.lines() {
        let mut temp_vec: Vec<u32> = vec![];
        instruction.split_whitespace().for_each(|word| {
            if word.parse::<u32>().is_ok() {
                temp_vec.push(word.parse::<u32>().unwrap());
            }
        });

        let b = temp_vec
            .iter()
            .collect_tuple::<(&u32, &u32, &u32)>()
            .unwrap();
        computed_instructions.push((*b.0, *b.1, *b.2));
    }

    println!("{:?}", computed_stacks);
    // println!("{:?}", computed_instructions);

    for (count, from, to) in computed_instructions.iter() {
        println!("");
        println!("{} {} {}", count, from, to);
        let from_index = (from - 1) as usize;
        let to_index = (to - 1) as usize;
        let split_index = computed_stacks[from_index].len() - (*count) as usize;

        let mut items = computed_stacks[from_index].split_off(split_index);
        // Part 2 Change 👇
        // items.reverse();
        println!("{:?}", items);
        computed_stacks[to_index].append(&mut items);
        println!("{:?}", computed_stacks);
    }

    let a = computed_stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .join("");

    a
}

fn part_2(input: &String) -> u32 {
    input;
    0
}
