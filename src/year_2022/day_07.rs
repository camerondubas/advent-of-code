use std::collections::HashMap;

use itertools::Itertools;

fn change_dir<'a>(path: &'a str, current_dir: Vec<&'a str>) -> Vec<&'a str> {
    match path {
        "/" => vec!["/"],
        ".." => {
            let mut new_dir = current_dir.clone();
            new_dir.pop();
            new_dir
        }
        _ => {
            let mut new_dir = current_dir.clone();
            new_dir.push(path);
            new_dir
        }
    }
}

fn is_command(line_segment: Option<&&str>) -> bool {
    line_segment.unwrap().eq(&"$")
}

fn parse_input(input: &String) -> Vec<(Vec<&str>, u32)> {
    let mut current_dir = vec!["/"];
    let mut lines = input.lines().peekable();
    let mut files: Vec<(Vec<&str>, u32)> = vec![];

    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        let mut segments = line.split_whitespace().peekable();

        if !is_command(segments.peek()) {
            continue;
        }

        // Consume the '$' character
        segments.next();

        match segments.next().unwrap() {
            "cd" => current_dir = change_dir(segments.next().unwrap(), current_dir.clone()),
            "ls" => {
                while lines.peek().is_some() && !lines.peek().unwrap().starts_with('$') {
                    let line = lines.next().unwrap();
                    if line.starts_with("dir") {
                        continue;
                    }

                    let file_size = line
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    files.push((current_dir.clone(), file_size));
                }
            }
            _ => panic!("Unknown command"),
        }
    }

    files
}

fn compute_dirs(files: Vec<(Vec<&str>, u32)>) -> HashMap<String, u32> {
    let mut dirs: HashMap<String, u32> = HashMap::new();

    for file in files.clone().iter() {
        let mut dir_path = String::from("");
        for seg in file.0.iter() {
            dir_path += &format!("/{}", &seg);
            dirs.insert(dir_path.clone(), dirs.get(&dir_path).unwrap_or(&0) + file.1);
        }
    }

    dirs
}

fn part_1(input: &String) -> usize {
    let files = parse_input(input);
    let dirs = compute_dirs(files);
    let max_size = 100_000;

    let sum: u32 = dirs
        .iter()
        .filter(|(_, size)| **size <= max_size)
        .map(|(_, size)| size)
        .sum();

    sum as usize
}

fn part_2(input: &String) -> usize {
    let files = parse_input(input);
    let dirs = compute_dirs(files);

    let drive_size = 70_000_000;
    let space_needed = 30_000_000;
    let available_space = drive_size - dirs.get("//").unwrap_or(&0);
    let target_dir_size = space_needed - available_space;

    let dir_size = dirs
        .iter()
        .filter(|(_, size)| **size >= target_dir_size)
        .map(|(_, size)| size)
        .sorted()
        .next()
        .unwrap();

    *dir_size as usize
}

#[cfg(test)]
mod tests {
    use crate::get_input;

    use super::*;

    #[test]
    fn test_part_1_dummy() {
        let input = get_input("2022", "07", Some("dummy"));
        let output = part_1(&input);
        assert_eq!(output, 95437);
    }

    #[test]
    fn test_part_1() {
        let input = get_input("2022", "07", None);
        let output = part_1(&input);
        assert_eq!(output, 1391690);
    }

    #[test]
    fn test_part_2_dummy() {
        let input = get_input("2022", "07", Some("dummy"));
        let output = part_2(&input);
        assert_eq!(output, 24933642);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("2022", "07", None);
        let output = part_2(&input);
        assert_eq!(output, 5469168);
    }
}
