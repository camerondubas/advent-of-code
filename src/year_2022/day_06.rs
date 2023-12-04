use itertools::Itertools;

fn get_position(input: &String, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .find_position(|window| window.iter().all_unique())
        .unwrap()
        .0
        + size
}
fn part_1(input: &String) -> usize {
    let packet_size = 4;
    get_position(input, packet_size)
}

fn part_2(input: &String) -> usize {
    let message_size = 14;
    get_position(input, message_size)
}

fn part_2_bytes(input: &String) -> usize {
    let message_size = 14;
    input
        .as_bytes()
        .windows(message_size)
        .find_position(|window| window.iter().all_unique())
        .unwrap()
        .0
        + message_size
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2022", "06", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 5);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2022", "06", None);
        let output = part_1(&input);
        assert_eq!(output, 1210);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2022", "06", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 23);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2022", "06", None);
        let output = part_2(&input);
        assert_eq!(output, 3476);
    }

    #[test]
    fn test_part_2_bytes() {
        let input = get_input("2022", "06", None);
        let output = part_2_bytes(&input);
        assert_eq!(output, 3476);
    }
}
