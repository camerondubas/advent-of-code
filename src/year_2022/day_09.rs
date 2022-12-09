use std::{collections::HashSet, vec};

use crate::runner::run;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance(&self, pos: &Position) -> (i32, i32) {
        (self.x - pos.x, self.y - pos.y)
    }
}

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

fn at_most_one(num: i32) -> i32 {
    if num == 0 {
        return 0;
    }

    if num.is_positive() {
        return 1;
    }

    return -1;
}

fn get_next_knot_position(tail: &Position, head: &Position) -> Position {
    let (x, y) = head.distance(tail);
    let mut new_tail = tail.clone();

    if x.abs() >= 2 || y.abs() >= 2 {
        new_tail.x += at_most_one(x);
        new_tail.y += at_most_one(y);
    }

    new_tail
}

fn snake(input: &String, knot_count: usize) -> usize {
    let mut knots = vec![Position::new(0, 0); knot_count];
    let mut tail_positions = HashSet::new();

    for line in input.lines() {
        let (dir, dist) = line.split_whitespace().collect_tuple().unwrap();
        let dist = dist.parse::<i32>().unwrap();

        for _ in 0..dist {
            match dir {
                "R" => knots[0].x += 1,
                "L" => knots[0].x -= 1,
                "U" => knots[0].y += 1,
                "D" => knots[0].y -= 1,
                _ => panic!("Unknown direction"),
            }

            let mut prev_knot = knots[0];

            for (idx, knot) in knots.clone().iter().enumerate() {
                let new_knot = get_next_knot_position(knot, &prev_knot);
                knots[idx] = new_knot;
                prev_knot = new_knot;
            }

            let tail_position = knots[knots.len() - 1];
            tail_positions.insert(tail_position);
            // draw_grid(&knots);
        }
    }
    tail_positions.len()
}

fn part_1(input: &String) -> usize {
    snake(input, 2)
}

fn part_2(input: &String) -> usize {
    snake(input, 10)
}

// // For visually debugging the grid
// fn draw_grid(knots: &Vec<Position>) {
//     println!("");
//     let mut grid: Vec<Vec<&str>> = vec![vec!["."; 10]; 10];
//     for (idx, knot) in knots.iter().enumerate().collect_vec().clone() {
//         let x = match idx {
//             0 => "H",
//             1 => "1",
//             2 => "2",
//             3 => "3",
//             4 => "4",
//             5 => "5",
//             6 => "6",
//             7 => "7",
//             8 => "8",
//             9 => "9",
//             _ => panic!("Unknown knot"),
//         };
//         grid[knot.y as usize][knot.x as usize] = x;
//     }
//     for row in grid.iter().rev().collect_vec().clone() {
//         println!("{:?}", row.join(""));
//     }
// }
