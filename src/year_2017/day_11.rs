use std::cmp::Ordering;

use crate::runner::run;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

fn count_steps(coords: (i32, i32)) -> usize {
    let mut steps = 0;

    let mut x = coords.0;
    let mut y = coords.1;

    while x != 0 {
        match y.cmp(&0) {
            Ordering::Equal => break,
            Ordering::Greater => {
                if x > 0 {
                    // ne (1, 1)
                    x -= 1;
                    y -= 1;
                } else {
                    // nw (-1, 1)
                    x += 1;
                    y -= 1;
                }
                steps += 1;
            }
            Ordering::Less => {
                if x > 0 {
                    // se (1, -1)
                    x -= 1;
                    y += 1;
                } else {
                    // sw (-1, -1)
                    x += 1;
                    y += 1;
                }
                steps += 1;
            }
        }
    }

    while y != 0 {
        match y.cmp(&0) {
            Ordering::Equal => break,
            Ordering::Greater => {
                y -= 2;
                steps += 1;
            }
            Ordering::Less => {
                y += 2;
                steps += 1;
            }
        }
    }

    steps
}

fn part_1(input: &String) -> usize {
    let mut coords: (i32, i32) = (0, 0);

    input.split(',').for_each(|direction| {
        let val: (i32, i32) = match direction.trim() {
            "n" => (0, 2),
            "ne" => (1, 1),
            "nw" => (-1, 1),
            "s" => (0, -2),
            "se" => (1, -1),
            "sw" => (-1, -1),
            _ => panic!("Invalid direction"),
        };
        coords.0 += val.0;
        coords.1 += val.1;
    });

    count_steps(coords)
}

fn part_2(input: &String) -> usize {
    let mut coords: (i32, i32) = (0, 0);
    let mut furthest: usize = 0;

    input.split(',').for_each(|direction| {
        let val: (i32, i32) = match direction.trim() {
            "n" => (0, 2),
            "ne" => (1, 1),
            "nw" => (-1, 1),
            "s" => (0, -2),
            "se" => (1, -1),
            "sw" => (-1, -1),
            _ => panic!("Invalid direction"),
        };
        coords.0 += val.0;
        coords.1 += val.1;

        let current_steps = count_steps(coords);
        if current_steps > furthest {
            furthest = current_steps;
        }
    });

    furthest
}
