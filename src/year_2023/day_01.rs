const NUM_WORDS: [(&str, char); 18] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
    ("1", '1'),
    ("2", '2'),
    ("3", '3'),
    ("4", '4'),
    ("5", '5'),
    ("6", '6'),
    ("7", '7'),
    ("8", '8'),
    ("9", '9'),
];

pub fn part_1(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let mut line_chars = line.chars();
        let first = line_chars.find(|c| c.is_ascii_digit()).unwrap();
        let last = line_chars.rfind(|c| c.is_ascii_digit()).unwrap_or(first);

        acc + format!("{}{}", first, last).parse::<u32>().unwrap()
    })
}

pub fn part_2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut first = (usize::MAX, '_');
        let mut last = (usize::MIN, '_');

        for (word, value) in NUM_WORDS {
            let matches: Vec<_> = line.match_indices(word).collect();
            for (idx, _) in matches {
                if idx < first.0 {
                    first.0 = idx;
                    first.1 = value;
                }
                if idx >= last.0 {
                    last.0 = idx;
                    last.1 = value;
                }
            }
        }

        sum += format!("{}{}", first.1, last.1).parse::<u32>().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2023", "01", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 142);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2023", "01", None);
        let output = part_1(&input);
        assert_eq!(output, 54968);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2023", "01", Some("dummy-2"));
        let output = part_2(&input);
        assert_eq!(output, 281);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2023", "01", None);
        let output = part_2(&input);
        assert_eq!(output, 54094);
    }
}
