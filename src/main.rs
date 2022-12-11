use std::env;
use std::fs;

extern crate aoc;

fn parse_input(args: &Vec<String>) -> String {
    let mut input = String::from("input");
    if args.len().ge(&4) {
        match args[3].as_str() {
            "viz" => (),
            _ => {
                input.push('-');
                input.push_str(&args[3]);
            }
        }
    };
    input
}

fn parse_viz(args: &Vec<String>) -> bool {
    if args.len().eq(&4) {
        return args[3].as_str() == "viz";
    };

    if args.len().ge(&5) {
        return args[4].as_str() == "viz";
    };

    return false;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = &args[1].as_str();
    let day = &args[2].as_str();
    let input = parse_input(&args);
    let is_viz = parse_viz(&args);

    let file_path = format!("./{}/{}/{}.txt", year, day, input);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Insert empty line to make the output look nicer
    println!("");

    if is_viz {
        aoc::runner::run_viz(*year, *day, contents);
    } else {
        aoc::runner::run_day(*year, *day, contents);
    }
}
