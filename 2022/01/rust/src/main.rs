use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("=== Part 1 ===");
    println!("Solution: {:?}", part_1(&contents));

    println!("=== Part 2 ===");
    println!("Solution: {:?}", part_2(&contents));
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
