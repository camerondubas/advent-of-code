use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, Default)]
struct Part {
    number: u32,
    range: Vec<usize>,
}

impl Part {
    fn new(part: &(String, Vec<usize>)) -> Self {
        Self {
            number: part.0.parse::<u32>().unwrap(),
            range: part.1.clone(),
        }
    }

    fn adjacents(&self, line_len: usize) -> HashSet<usize> {
        let mut adjacents: HashSet<usize> = HashSet::new();

        for pos in &self.range {
            let left_edge = pos % line_len == 0;
            let right_edge = pos % line_len == line_len - 1;
            let mut all_adjacents: Vec<Option<usize>> = vec![
                pos.checked_sub(line_len), // top
                pos.checked_add(line_len), // bottom
            ];

            if !left_edge {
                all_adjacents.extend([
                    pos.checked_sub(line_len - 1), // top left
                    pos.checked_sub(1),            // left
                    pos.checked_add(line_len - 1), // bottom left
                ]);
            }
            if !right_edge {
                all_adjacents.extend([
                    pos.checked_sub(line_len + 1), // top right
                    pos.checked_add(1),            // right
                    pos.checked_add(line_len + 1), // bottom right
                ]);
            }

            let add_adjacents = all_adjacents.iter().filter_map(|x| *x);

            adjacents.extend(add_adjacents)
        }

        adjacents
    }
}

#[derive(Debug, Default, Clone)]
struct Gear {
    index: usize,
    parts: Vec<u32>,
}
impl Gear {
    fn ratio(&self) -> u32 {
        let mut ratio = 1;
        for part in &self.parts {
            ratio *= part;
        }
        ratio
    }
}

#[derive(Debug, Default)]
struct Engine {
    line_len: usize,
    parts: Vec<Part>,
    symbols: Vec<usize>,
    possible_gears: Vec<Gear>,
}

impl Engine {
    fn new(input: &str) -> Self {
        let line_len = input.lines().next().unwrap().len();

        let mut partial_part: Option<(String, Vec<usize>)> = None;
        let mut parts: Vec<Part> = vec![];
        let mut symbols: Vec<usize> = vec![];
        let mut possible_gears: Vec<Gear> = vec![];

        for (line_idx, line) in input.lines().enumerate() {
            for (char_idx, c) in line.char_indices() {
                let pos = char_idx + line_idx * line_len;
                if c.is_ascii_digit() {
                    if let Some(ref mut part) = partial_part {
                        part.0.push(c);
                        part.1.push(pos);
                    } else {
                        partial_part = Some((c.to_string(), vec![pos]));
                    }
                    continue;
                }

                if let Some(part) = &partial_part {
                    parts.push(Part::new(part));
                    partial_part = None;
                }

                if c != '.' {
                    symbols.push(pos);
                    if c == '*' {
                        possible_gears.push(Gear {
                            index: pos,
                            parts: vec![],
                        });
                    }
                }
            }
            if let Some(part) = &partial_part {
                parts.push(Part::new(part));
                partial_part = None;
            }
        }

        Self {
            line_len,
            parts,
            symbols,
            possible_gears,
        }
    }
    fn valid_parts(&self) -> Vec<&Part> {
        let mut valid_parts: Vec<&Part> = vec![];

        self.parts.iter().for_each(|part| {
            let is_adjacent_to_symbol = part
                .adjacents(self.line_len)
                .iter()
                .any(|adjacent| self.symbols.iter().any(|symbol| symbol == adjacent));

            if is_adjacent_to_symbol {
                valid_parts.push(part);
            }
        });

        valid_parts
    }

    fn gears(&mut self) -> Vec<Gear> {
        self.parts.iter().for_each(|part| {
            for adjacent in part.adjacents(self.line_len) {
                for gear in &mut self.possible_gears {
                    if gear.index == adjacent {
                        gear.parts.push(part.number)
                    }
                }
            }
        });

        self.possible_gears
            .iter()
            .filter(|gear| gear.parts.len() > 1)
            .cloned()
            .collect_vec()
    }
}

pub fn part_1(input: &str) -> u32 {
    let engine = Engine::new(input);

    engine
        .valid_parts()
        .iter()
        .map(|part| part.number)
        .sum::<u32>()
}

pub fn part_2(input: &str) -> u32 {
    let mut engine = Engine::new(input);

    engine.gears().iter().map(|gear| gear.ratio()).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2023", "03", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 4361);
    }

    #[test]
    fn test_part_1_case_1() {
        let input = "........
.24..4..
......*.";
        let output = part_1(input);
        assert_eq!(output, 4);
    }

    #[test]
    fn test_part_1_case_2() {
        let input = "........
.24$-4..
......*.";
        let output = part_1(input);
        assert_eq!(output, 28);
    }

    #[test]
    fn test_part_1_case_3() {
        let input = "11....11
..$..$..
11....11";
        let output = part_1(input);
        assert_eq!(output, 44);
    }

    #[test]
    fn test_part_1_case_4() {
        let input = "$......$
.1....1.
.1....1.
$......$";
        let output = part_1(input);
        assert_eq!(output, 4);
    }
    #[test]
    fn test_part_1_case_5() {
        let input = "$......$
.11..11.
.11..11.
$......$";
        let output = part_1(input);
        assert_eq!(output, 44);
    }
    #[test]
    fn test_part_1() {
        let input = get_input("2023", "03", None);
        let output = part_1(&input);
        assert_eq!(output, 554003);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2023", "03", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 467835);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2023", "03", None);
        let output = part_2(&input);
        assert_eq!(output, 87263515);
    }
}
