use pathfinding::prelude::astar;
use pathfinding::prelude::bfs;

use std::fmt::Display;

use crate::runner::run;
use colored::Colorize;
use itertools::Itertools;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    // run("Part 2", part_2, &contents);
}

fn can_move_to_point(target: u32, current: u32) -> bool {
    if target <= current {
        return true;
    }
    return target - current <= 1;
}

fn get_char_code(char: char) -> u32 {
    let num = char as u32;
    let offset = if char.is_uppercase() { 64 - 26 } else { 96 };
    return num - offset;
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
    value: u32,
    is_start: bool,
    is_end: bool,
}

impl Point {
    fn new(x: usize, y: usize, value: u32, is_start: bool, is_end: bool) -> Point {
        Point {
            x,
            y,
            value,
            is_start,
            is_end,
        }
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self.value {
            _ => format!("{:0>2}", self.value).bold(),
        };

        if f.sign_minus() {
            return write!(f, " {} ", out);
        }

        write!(f, " {},{} {} ", self.x, self.y, out)
    }
}

struct Grid {
    points: Vec<Vec<Point>>,
    start: Point,
    end: Point,
}

impl Grid {
    fn new(points: Vec<Vec<Point>>) -> Grid {
        let start = points
            .iter()
            .flatten()
            .find(|point| point.is_start)
            .unwrap()
            .clone();
        let end = points
            .iter()
            .flatten()
            .find(|point| point.is_end)
            .unwrap()
            .clone();
        Grid { points, start, end }
    }

    fn get_neighbors(&self, point: &Point) -> Vec<&Point> {
        let mut neighbors = Vec::new();
        let x = point.x;
        let y = point.y;

        if y > 0 {
            let up = &self.points[y - 1][x];
            if can_move_to_point(up.value, point.value) {
                neighbors.push(up);
            }
        }

        if x > 0 {
            let left = &self.points[y][x - 1];
            if can_move_to_point(left.value, point.value) {
                neighbors.push(left);
            }
        }
        if x < self.points[y].len() - 1 {
            let right = &self.points[y][x + 1];
            if can_move_to_point(right.value, point.value) {
                neighbors.push(right);
            }
        }
        if y < self.points.len() - 1 {
            let down = &self.points[y + 1][x];
            if can_move_to_point(down.value, point.value) {
                neighbors.push(down);
            }
        }

        neighbors
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.points.iter().for_each(|row| {
            row.iter().for_each(|point| {
                print!("{:-}", point);
            });
            print!("\n");
        });
        write!(f, "")
    }
}

fn part_1(input: &String) -> u32 {
    let _grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    let val = match char {
                        'S' => get_char_code('a') - 1,
                        'E' => get_char_code('z') + 1,
                        _ => get_char_code(char),
                    };
                    Point::new(x, y, val, char == 'S', char == 'E')
                })
                .collect_vec()
        })
        .collect_vec();

    let grid = Grid::new(_grid);

    let path = astar(
        &grid.start,
        |point| {
            grid.get_neighbors(point)
                .into_iter()
                .map(|p| (p.clone(), 1))
                .collect_vec()
        },
        |point| {
            let delta_x = (grid.end.x as i32 - point.x as i32).abs();
            let delta_y = (grid.end.y as i32 - point.y as i32).abs();
            let h = (delta_x + delta_y) as u32;
            h
        },
        |point| point == &grid.end,
    );

    println!("{:?}", path.expect("No path found").1);
    // pathfind(&grid);
    0
}

#[derive(Debug, Clone)]
struct APoint {
    point: Point,
    parent: Option<Box<APoint>>,
    g: u32,
    h: u32,
    f: u32,
}

impl APoint {
    fn from(point: &Point, current_a_point: &APoint, end: &Point) -> APoint {
        let delta_x = (end.x as i32 - point.x as i32).abs();
        let delta_y = (end.y as i32 - point.y as i32).abs();
        let g = current_a_point.g + 1;
        let h = (delta_x + delta_y) as u32;

        let a_point = APoint {
            point: point.clone(),
            parent: Some(Box::new(current_a_point.clone())),
            g,
            h,
            f: g + h,
        };
        a_point
    }
}

impl Display for APoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{} g:{} h:{} f:{}",
            self.point.x,
            self.point.y,
            self.g,
            self.h,
            self.g + self.h
        )
    }
}

// fn pathfind(grid: &Grid) {
//     println!("Pathfinding...");
//     let mut closed_list: Vec<APoint> = Vec::new();
//     let mut open_list: Vec<APoint> = Vec::new();
//     let mut _steps = 0;

//     let delta_x = (grid.end.x as i32 - grid.start.x as i32).abs();
//     let delta_y = (grid.end.y as i32 - grid.start.y as i32).abs();
//     let g = 0;
//     let h = (delta_x + delta_y) as u32;

//     let current_point = APoint {
//         point: grid.start,
//         parent: None,
//         g,
//         h,
//         f: g + h,
//     };

//     println!("start: {:?}", current_point);

//     //initialize
//     open_list.push(current_point.clone());
//     let mut lowest_f;

//     while closed_list.last().is_none() || closed_list.last().unwrap().point != grid.end {
//         _steps += 1;
//         let mut temp_list = open_list.clone();
//         temp_list.sort_by(|a, b| b.f.cmp(&a.f));

//         println!("{}", open_list.len());
//         if open_list.is_empty() {
//             println!("No path found, {} steps", _steps);
//             let last = closed_list.last().unwrap();
//             let parent = last.parent.as_ref().unwrap();
//             println!("first: {:?}", closed_list.first().unwrap().point);
//             println!("closed_list.last(): {:?}, f: {}", last.point, last.f);
//             println!("p: {:?}, f: {}", parent.point, parent.f);

//             // println!("Closed List:");
//             // let mut last = closed_list.last().unwrap();
//             // let mut steps = 0;
//             // while last.parent.is_some() {
//             //     println!("{:?}, f: {}", last.point, last.f);
//             //     let parent = last.parent.as_ref().unwrap();
//             //     last = parent;
//             //     steps += 1;
//             // }
//             // println!("Steps: {}", steps);
//             return;
//         }

//         let lowest_idx = open_list
//             .iter()
//             .position(|i| i.point == temp_list.last().unwrap().point)
//             .unwrap();
//         lowest_f = open_list.remove(lowest_idx);
//         closed_list.push(lowest_f.clone());

//         grid.get_neighbors(&lowest_f.point)
//             .iter()
//             .for_each(|point| {
//                 if closed_list.iter().any(|i| i.point == *point) {
//                     return;
//                 }

//                 let maybe_open_list_point = open_list.iter_mut().find(|i| i.point == *point);
//                 if maybe_open_list_point.is_some() {
//                     let open_list_point = maybe_open_list_point.unwrap();
//                     let a_point = APoint::from(point, &lowest_f, &grid.end);
//                     if a_point.f < open_list_point.f {
//                         open_list_point.f = a_point.f;
//                         open_list_point.parent = a_point.parent;
//                     }
//                     return;
//                 }

//                 open_list.push(APoint::from(point, &lowest_f, &grid.end));
//             });
//     }

//     println!("Closed List:");
//     let mut last = closed_list.last().unwrap();
//     let mut steps = 0;
//     while last.parent.is_some() {
//         let parent = last.parent.as_ref().unwrap();
//         last = parent;
//         steps += 1;
//     }
//     println!("Steps: {}", steps);
// }

fn part_2(input: &String) -> u32 {
    0
}
