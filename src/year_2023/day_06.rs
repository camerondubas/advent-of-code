use itertools::Itertools;

#[derive(Debug)]
struct Race {
    distance: u128,
    time: u128,
}

impl Race {
    fn new(distance: u128, time: u128) -> Self {
        Self { distance, time }
    }

    fn get_all_outcomes(&self) -> Vec<u128> {
        let mut outcomes = Vec::new();
        for permutation in 0..=self.time {
            let hold = permutation;
            let distance = hold * (self.time - hold);

            outcomes.push(distance);
        }

        outcomes
    }

    /// Start in the middle and compute outcomes in one direction.
    /// Stop when an invalid (too short) outcome is found.
    /// This will be half of the total outcomes, since the outcomes are symmetrical.
    fn get_valid_outcomes_middle_out(&self) -> Vec<u128> {
        let half_time = self.time / 2;
        let mut outcomes = Vec::new();
        for permutation in (0..=half_time).rev() {
            let hold = permutation;
            let distance = hold * (self.time - hold);

            if distance > self.distance {
                outcomes.push(distance);
            } else {
                break;
            }
        }

        outcomes
    }

    /// Double the number of outcomes, since the outcomes are symmetrical.
    /// Subtract one if the time is even, to account for the middle outcome.
    fn count_valid_outcomes(&self) -> usize {
        let valid = self.get_valid_outcomes_middle_out().len() * 2;
        if self.time % 2 == 0 {
            valid - 1
        } else {
            valid
        }
    }
}

pub fn part_1(input: &str) -> u128 {
    let (times, distances) = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|num| num.parse::<u128>().unwrap())
        })
        .tuples()
        .next()
        .unwrap();

    let races = distances.zip(times).map(|(d, t)| Race::new(d, t));
    races.fold(1, |acc, race| acc * race.count_valid_outcomes() as u128)
}

pub fn part_1_slow(input: &str) -> u128 {
    let (times, distances) = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|num| num.parse::<u128>().unwrap())
        })
        .tuples()
        .next()
        .unwrap();

    let races = distances.zip(times).map(|(d, t)| Race::new(d, t));
    for race in races {
        println!("Race: {:?}", race);
        let o = race.get_all_outcomes();
        println!("Outcomes: {:?}", o);
        let a = o
            .iter()
            .map(|x| if x > &race.distance { x } else { &0 })
            .collect_vec();
        println!("OutcomeZ: {:?}", a);
    }

    todo!()
}

pub fn part_2(input: &str) -> u128 {
    let (times, distances) = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .fold(String::new(), |acc, x| acc + x)
                .parse::<u128>()
                .unwrap()
        })
        .tuples()
        .next()
        .unwrap();

    let race = Race::new(distances, times);
    race.count_valid_outcomes() as u128
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2023", "06", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 288);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2023", "06", None);
        let output = part_1(&input);
        assert_eq!(output, 449820);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2023", "06", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 71503);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2023", "06", None);
        let output = part_2(&input);
        assert_eq!(output, 42250895);
    }
}
