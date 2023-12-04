use itertools::Itertools;

fn parse_assignments(input: &str) -> Vec<(u32, u32, u32, u32)> {
    input
        .lines()
        .map(|assignment| {
            assignment
                .split(',')
                .flat_map(|range| range.split('-').collect::<Vec<&str>>())
                .map(|num| num.parse::<u32>().unwrap())
                .collect_tuple::<(u32, u32, u32, u32)>()
                .unwrap()
        })
        .collect()
}

pub fn part_1(input: &str) -> u32 {
    parse_assignments(input)
        .iter()
        .fold(0, |acc, (a1, a2, b1, b2)| {
            if ((a1..=a2).contains(&b1) && (a1..=a2).contains(&b2))
                || ((b1..=b2).contains(&a1) && (b1..=b2).contains(&a2))
            {
                acc + 1
            } else {
                acc
            }
        })
}

pub fn part_2(input: &str) -> u32 {
    parse_assignments(input)
        .iter()
        .fold(0, |acc, (a1, a2, b1, b2)| {
            if (*a1..=*a2).any(|a| (*b1..=*b2).contains(&a)) {
                acc + 1
            } else {
                acc
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2022", "04", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 2);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2022", "04", None);
        let output = part_1(&input);
        assert_eq!(output, 444);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2022", "04", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 4);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2022", "04", None);
        let output = part_2(&input);
        assert_eq!(output, 801);
    }
}
