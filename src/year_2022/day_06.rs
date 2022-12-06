use crate::runner::run;
use itertools::Itertools;

pub fn solution(contents: String) {
    run("Part 1", part_1, &contents);
    run("Part 2", part_2, &contents);
    run("Part 2 Bytes", part_2_bytes, &contents);
}

fn get_window_index(input: &String, size: usize) -> usize {
    input
        .as_bytes()
        .windows(size)
        .find_position(|window| window.iter().all_unique())
        .unwrap()
        .0
}
fn part_1(input: &String) -> usize {
    let packet_size = 4;
    get_window_index(input, packet_size) + packet_size
}

fn part_2(input: &String) -> usize {
    let packet_size = 14;
    get_window_index(input, packet_size) + packet_size
}

fn part_2_bytes(input: &String) -> usize {
    let packet_size = 14;
    input
        .as_bytes()
        .windows(packet_size)
        .find_position(|window| window.iter().all_unique())
        .unwrap()
        .0
        + packet_size
}
