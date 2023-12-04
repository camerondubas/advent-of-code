use crate::runner::run;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
}

fn part_1(input: &String) -> u32 {
    let mut largest = 0;

    input.split("\n\n").for_each(|group| {
        let sum = group
            .lines()
            .map(|n| n.parse::<u32>())
            .sum::<Result<u32, _>>()
            .unwrap();

        if sum > largest {
            largest = sum;
        }
    });

    largest
}

fn part_2(input: &String) -> u32 {
    let sums = input.split("\n\n").map(|group| {
        let sum = group
            .lines()
            .map(|n| n.parse::<u32>())
            .sum::<Result<u32, _>>()
            .unwrap();

        sum
    });

    let mut sum_vec = sums.collect::<Vec<u32>>();
    sum_vec.sort_by(|a, b| b.cmp(a));
    sum_vec[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2022", "01", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 24000);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2022", "01", None);
        let output = part_1(&input);
        assert_eq!(output, 74711);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2022", "01", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 45000);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2022", "01", None);
        let output = part_2(&input);
        assert_eq!(output, 209481);
    }
}
