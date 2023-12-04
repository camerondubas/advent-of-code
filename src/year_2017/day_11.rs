use std::cmp::Ordering;

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

pub fn part_1(input: &str) -> usize {
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

pub fn part_2(input: &str) -> usize {
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

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2017", "11", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 3);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2017", "11", None);
        let output = part_1(&input);
        assert_eq!(output, 824);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2017", "11", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 5);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2017", "11", None);
        let output = part_2(&input);
        assert_eq!(output, 1548);
    }
}
