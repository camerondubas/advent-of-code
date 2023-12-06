use itertools::Itertools;
use rayon::prelude::*;
use std::ops::Range;

struct SeedMapList {
    seed_maps: Vec<SeedMap>,
}

struct SeedMap {
    ranges: Vec<SeedMapRange>,
}

struct SeedMapRange {
    seed_range: Range<u64>,
    dest_offset: u64,
}

impl SeedMapList {
    fn from_sections(sections: Vec<&str>) -> SeedMapList {
        let maps = sections
            .iter()
            .map(|section| {
                let ranges = section
                    .lines()
                    .skip(1)
                    .map(|line| {
                        let (dest_start, src_start, range) = line
                            .split_whitespace()
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect_tuple()
                            .unwrap();

                        SeedMapRange {
                            seed_range: src_start..src_start + range,
                            dest_offset: dest_start,
                        }
                    })
                    .collect_vec();
                SeedMap { ranges }
            })
            .collect_vec();
        SeedMapList { seed_maps: maps }
    }

    fn find_location(&self, seed: u64) -> u64 {
        self.seed_maps
            .iter()
            .fold(seed, |acc, map| map.find_location(&acc))
    }
}

impl SeedMap {
    fn find_location(&self, seed: &u64) -> u64 {
        self.ranges.iter().fold(*seed, |acc, map| {
            if map.seed_range.contains(seed) {
                let offset = seed - map.seed_range.start;
                let dest = map.dest_offset + offset;
                return dest;
            }
            acc
        })
    }
}

pub fn part_1(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let seeds_str = sections.next().unwrap().split_once(": ").unwrap().1;
    let seeds = seeds_str
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let seed_map_list = SeedMapList::from_sections(sections.collect_vec());

    seeds
        .par_iter()
        .map(|seed| seed_map_list.find_location(*seed))
        .min()
        .unwrap()
}

pub fn part_2(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let seeds_str = sections.next().unwrap().split_once(": ").unwrap().1;
    let seeds = seeds_str
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .tuples()
        .map(|(start, length)| (start..start + length))
        .flat_map(|range| range.clone());

    let seed_map_list = SeedMapList::from_sections(sections.collect_vec());

    seeds
        .par_bridge()
        .map(|seed| seed_map_list.find_location(seed))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2023", "05", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 35);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2023", "05", None);
        let output = part_1(&input);
        assert_eq!(output, 175622908);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2023", "05", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 46);
    }

    #[ignore = "Takes multiple minutes"]
    #[test]
    fn test_part_2() {
        let input = get_input("2023", "05", None);
        let output = part_2(&input);
        assert_eq!(output, 5200543);
    }
}
