use itertools::Itertools;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Clone, Debug, Default)]
struct Round {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Round {
    fn is_valid(&self) -> bool {
        self.red <= MAX_RED && self.green <= MAX_GREEN && self.blue <= MAX_BLUE
    }
}

impl From<&str> for Round {
    fn from(val: &str) -> Self {
        val.split(", ")
            .map(|color| color.split_once(' ').unwrap())
            .fold(Round::default(), |mut acc, (value, color)| {
                let value = value.parse().unwrap();
                match color {
                    "red" => acc.red = value,
                    "green" => acc.green = value,
                    "blue" => acc.blue = value,
                    _ => panic!("Invalid color"),
                };
                acc
            })
    }
}

#[derive(Debug)]
struct Game {
    pub id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn new(input: &str) -> Self {
        let (id_section, rounds_section) = input.split_once(": ").unwrap();
        let (_, id) = id_section.split_once(' ').unwrap();

        let rounds = rounds_section
            .split("; ")
            .map(|round| round.into())
            .collect_vec();

        Self {
            id: id.parse().unwrap(),
            rounds,
        }
    }

    fn is_valid(&self) -> bool {
        self.rounds.iter().all(|round| round.is_valid())
    }

    fn value(&self) -> Option<u32> {
        if self.is_valid() {
            Some(self.id)
        } else {
            None
        }
    }

    fn min_capacity(&self) -> [u32; 3] {
        self.rounds.iter().fold([0, 0, 0], |mut capacity, round| {
            capacity[0] = capacity[0].max(round.red);
            capacity[1] = capacity[1].max(round.green);
            capacity[2] = capacity[2].max(round.blue);
            capacity
        })
    }

    fn power(&self) -> u32 {
        self.min_capacity().into_iter().product()
    }
}

#[allow(clippy::all)]
pub fn part_1(input: &String) -> u32 {
    input
        .lines()
        .filter_map(|line| Game::new(line).value())
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    input.lines().map(|line| Game::new(line).power()).sum()
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2023", "02", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 8);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2023", "02", None);
        let output = part_1(&input);
        assert_eq!(output, 2593);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2023", "02", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 2286);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2023", "02", None);
        let output = part_2(&input);
        assert_eq!(output, 54699);
    }
}
