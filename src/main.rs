use std::env;
use std::fs;

extern crate aoc;

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = &args[1].as_str();
    let day = &args[2].as_str();
    let input = if args.len().gt(&3) && (&args[3]).eq("dummy") {
        "input-dummy"
    } else {
        "input"
    };
    let file_path = format!("./{}/{}/{}.txt", year, day, input);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Insert empty line to make the output look nicer
    println!("");

    match *year {
        "2022" => match *day {
            "01" => aoc::year_2022::day_01::solution(contents),
            "02" => aoc::year_2022::day_02::solution(contents),
            "03" => aoc::year_2022::day_03::solution(contents),
            "04" => aoc::year_2022::day_04::solution(contents),
            _ => println!("No solution for this day"),
        },
        _ => panic!("Year not implemented"),
    }
}
