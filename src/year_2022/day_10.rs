use std::vec;

use crate::runner::run;
use colored::Colorize;
use itertools::Itertools;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

fn part_1(input: &String) -> i32 {
    let mut register_x: i32 = 1;
    let mut cycle = 0;
    let mut signal_strength: i32 = 0;
    let mut loaded_instruction: Option<i32> = None;
    let mut instructions = format_input(input).into_iter().peekable();

    while instructions.peek().is_some() || loaded_instruction.is_some() {
        cycle += 1;

        if (cycle + 20) % 40 == 0 {
            signal_strength += cycle * register_x;
        }

        if loaded_instruction.is_none() {
            let instruction = instructions.next().unwrap();
            loaded_instruction = read_instruction(instruction);
        } else {
            register_x += loaded_instruction.unwrap();
            loaded_instruction = None;
        }
    }

    signal_strength
}

fn part_2(input: &String) -> String {
    let mut sprite_position: i32 = 1;
    let mut cycle: i32 = 0;
    let mut crt = vec![' '; 40 * 6];
    let mut loaded_instruction: Option<i32> = None;
    let mut instructions = format_input(input).into_iter().peekable();

    while instructions.peek().is_some() || loaded_instruction.is_some() {
        cycle += 1;

        let is_instruction_executing = loaded_instruction.is_some();

        if !is_instruction_executing {
            let instruction = instructions.next().unwrap();
            loaded_instruction = read_instruction(instruction);
        }

        update_crt(&mut crt, cycle, sprite_position);

        if is_instruction_executing {
            sprite_position += loaded_instruction.unwrap();
            loaded_instruction = None;
        }
    }
    draw_crt(&crt)
}

fn format_input(input: &String) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
        .collect_vec()
}

fn read_instruction(instruction: Vec<&str>) -> Option<i32> {
    match instruction[0] {
        "addx" => Some(instruction[1].parse::<i32>().unwrap()),
        "noop" => None,
        _ => panic!("Unknown instruction"),
    }
}

fn update_crt(crt: &mut Vec<char>, cycle_num: i32, sprite_position: i32) {
    let idx = (cycle_num - 1) as usize;

    let sprite_range = (sprite_position - 1)..=(sprite_position + 1);
    crt[idx] = match sprite_range.contains(&((idx as i32) % 40)) {
        true => '#',
        false => '.',
    };
}

fn draw_crt(crt: &Vec<char>) -> String {
    // let crt_lines = crt.chunks(40).map(|chunk| chunk.join("")).collect_vec();
    // for crt_line in crt_lines {
    //     println!("{}", crt_line);
    // }

    println!("");
    // let mut line = String::from("");
    // crt.iter().for_each(|c| {
    //     // let colored_char = match *char {
    //     //     "#" => " ".on_black(),
    //     //     _ => " ".on_green(),
    //     // };

    //     // line.push_str(&colored_char.to_string());
    // });

    let crt_lines2: Vec<String> = crt
        .chunks(40)
        .map(|c| c.iter().collect::<String>())
        .collect_vec();

    // let crt_lines2: Vec<String> = line
    //     .chars()
    //     .collect::<Vec<char>>()
    //     .chunks(40 * 10)
    //     .map(|c| c.iter().collect::<String>())
    //     .collect_vec();

    let mut output = String::from("");
    for crt_line in crt_lines2 {
        println!("{}", crt_line);
        output.push_str(&format!("{}\n", crt_line));
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2022", "10", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 13140);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2022", "10", None);
        let output = part_1(&input);
        assert_eq!(output, 12520);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2022", "10", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(
            output,
            String::from(
                "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....\n"
            )
            .trim_start()
        );
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2022", "10", None);
        let output = part_2(&input);
        assert_eq!(
            output,
            String::from(
                "
####.#..#.###..####.###....##..##..#....
#....#..#.#..#....#.#..#....#.#..#.#....
###..####.#..#...#..#..#....#.#....#....
#....#..#.###...#...###.....#.#.##.#....
#....#..#.#....#....#....#..#.#..#.#....
####.#..#.#....####.#.....##...###.####.\n"
            )
            .trim_start()
        );
    }
}
