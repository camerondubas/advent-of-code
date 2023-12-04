use crate::runner::run;
use itertools::Itertools;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

fn create_grid(input: &String) -> Vec<(usize, Vec<(usize, u32)>)> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .enumerate()
                .collect_vec()
        })
        .enumerate()
        .collect_vec()
}

fn check(list: Vec<(usize, u32)>) -> (bool, usize) {
    let height = list[0].1;
    let mut viewing_distance = 0;

    for item in list.iter().skip(1) {
        viewing_distance += 1;
        if item.1 >= height {
            return (false, viewing_distance);
        }
    }

    (true, viewing_distance)
}

fn get_up(grid: &Vec<(usize, Vec<(usize, u32)>)>, x: usize, y: usize) -> Vec<(usize, u32)> {
    grid[..y + 1].iter().map(|f| f.1[x]).rev().collect_vec()
}

fn get_down(grid: &Vec<(usize, Vec<(usize, u32)>)>, x: usize, y: usize) -> Vec<(usize, u32)> {
    grid[y..].iter().map(|f| f.1[x]).collect_vec()
}

fn get_left(grid: &Vec<(usize, Vec<(usize, u32)>)>, x: usize, y: usize) -> Vec<(usize, u32)> {
    grid[y].1[..x + 1].iter().copied().rev().collect_vec()
}

fn get_right(grid: &Vec<(usize, Vec<(usize, u32)>)>, x: usize, y: usize) -> Vec<(usize, u32)> {
    grid[y].1[x..].to_vec()
}

fn part_1(input: &String) -> usize {
    let grid = create_grid(input);

    let size = (grid[0].1.len(), grid.len());
    let mut visible_sum = size.0 * 2 + size.1 * 2 - 4;

    for (y, line) in grid.clone() {
        if y == 0 || y == size.1 - 1 {
            continue;
        }
        for (x, _) in line {
            if x == 0 || x == size.0 - 1 {
                continue;
            }

            if check(get_up(&grid, x, y)).0
                || check(get_down(&grid, x, y)).0
                || check(get_left(&grid, x, y)).0
                || check(get_right(&grid, x, y)).0
            {
                visible_sum += 1;
            }
        }
    }
    visible_sum
}

fn part_2(input: &String) -> usize {
    let grid = create_grid(input);
    let mut highest_scenic_score = 0;

    for (y, line) in grid.clone() {
        for (x, _) in line {
            let scenic_score = check(get_up(&grid, x, y)).1
                * check(get_left(&grid, x, y)).1
                * check(get_right(&grid, x, y)).1
                * check(get_down(&grid, x, y)).1;

            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }
    highest_scenic_score
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2022", "08", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 21);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2022", "08", None);
        let output = part_1(&input);
        assert_eq!(output, 1779);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2022", "08", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 8);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2022", "08", None);
        let output = part_2(&input);
        assert_eq!(output, 172224);
    }
}
