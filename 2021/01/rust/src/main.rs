use std::env;
use std::fs;
use std::str::Lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let lines = contents.lines();
    let lines2 = contents.lines();

    println!("=== Part 1 ===");
    part_1(lines);
    println!("=== Part 2 ===");
    part_2(lines2);
}

fn part_1(lines: Lines) {
    let depths = lines;

    let mut previous_depth = 0;
    let mut increase_count = 0;

    for depth in depths {
        let depth_int = depth.parse::<i32>().unwrap();
        if previous_depth == 0 {
            previous_depth = depth_int;
            continue;
        }

        if depth_int > previous_depth {
            increase_count += 1;
        }

        previous_depth = depth_int;
    }

    println!("{}", increase_count);
}

fn part_2(lines: Lines) {
    let windows = lines
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<i32>>();

    let mut previous_depth = 0;
    let mut increase_count = 0;

    for depth in &windows {
        if previous_depth == 0 {
            previous_depth = *depth;
            continue;
        }

        if *depth > previous_depth {
            increase_count += 1;
        }

        previous_depth = *depth;
    }

    println!("{}", increase_count);
}
