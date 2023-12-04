fn get_offset(is_uppercase: bool) -> u32 {
    if is_uppercase {
        64 - 26
    } else {
        96
    }
}

pub fn part_1(input: &String) -> u32 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .fold(0, |acc, (left, right)| {
            let duplicate = left
                .chars()
                .find(|lc| right.chars().any(|rc| rc.eq(lc)))
                .unwrap();
            acc + (duplicate as u32) - get_offset(duplicate.is_uppercase())
        })
}

pub fn part_2(input: &String) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    lines.chunks(3).fold(0, |acc, group| {
        let badge = group[0]
            .chars()
            .find(|c| group[1].chars().any(|c2| c2.eq(c)) && group[2].chars().any(|c2| c2.eq(c)))
            .unwrap();

        acc + (badge as u32) - get_offset(badge.is_uppercase())
    })
}

pub fn part_2_sorted(input: &String) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    lines.chunks(3).fold(0, |acc, group| {
        let mut clone = group.to_vec();
        clone.sort();
        let badge = clone[0]
            .chars()
            .find(|c| group[1].chars().any(|c2| c2.eq(c)) && group[2].chars().any(|c2| c2.eq(c)))
            .unwrap();

        acc + (badge as u32) - get_offset(badge.is_uppercase())
    })
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2022", "03", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 157);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2022", "03", None);
        let output = part_1(&input);
        assert_eq!(output, 8153);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2022", "03", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 70);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2022", "03", None);
        let output = part_2(&input);
        assert_eq!(output, 2342);
    }

    #[test]
    fn test_part_2_dummy_sorted() {
        let input = get_input("2022", "03", Some("dummy"));
        let output = part_2_sorted(&input);
        assert_eq!(output, 78);
    }

    #[test]
    fn test_part_2_sorted() {
        let input = get_input("2022", "03", None);
        let output = part_2_sorted(&input);
        assert_eq!(output, 2985);
    }
}
