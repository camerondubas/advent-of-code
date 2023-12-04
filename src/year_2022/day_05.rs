use itertools::Itertools;

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
pub fn part_1(input: &str) -> String {
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

pub fn part_2(input: &str) -> String {
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

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2022", "05", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, "CMZ");
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2022", "05", None);
        let output = part_1(&input);
        assert_eq!(output, "PSNRGBTFT");
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2022", "05", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, "MCD");
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2022", "05", None);
        let output = part_2(&input);
        assert_eq!(output, "BNTZFPMMW");
    }
}
