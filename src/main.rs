use std::env;
use std::fs;

extern crate aoc;

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = &args[1].as_str();
    let day = &args[2].as_str();
    let input = if (&args[3]).eq("dummy") {
        "input-dummy"
    } else {
        "input"
    };
    let file_path = format!("./{}/{}/{}.txt", year, day, input);
    println!("file_path: {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    match *year {
        "2022" => match *day {
            "03" => aoc::year_2022::day_03::solution(contents),
            _ => println!("No solution for this day"),
        },
        _ => panic!("Year not implemented"),
    }
}
