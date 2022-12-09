use std::env;
use std::fs;

extern crate aoc;

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = &args[1].as_str();
    let day = &args[2].as_str();
    let mut input = String::from("input");
    if args.len().gt(&3) {
        input.push('-');
        input.push_str(&args[3]);
    };
    let file_path = format!("./{}/{}/{}.txt", year, day, input);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Insert empty line to make the output look nicer
    println!("");

    aoc::runner::run_day(*year, *day, contents);
}
