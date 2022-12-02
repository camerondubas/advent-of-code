use crate::runner::run;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
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
