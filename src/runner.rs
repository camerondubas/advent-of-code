use colored::Colorize;
use std::{fmt::Debug, time::Instant};

use crate::{viz, year_2017, year_2022};

pub fn run<T>(name: &str, func: fn(&String) -> T, input: &String)
where
    T: Debug,
{
    println!("=== {} ===", format!("{}", name).bold());
    let start = Instant::now();
    let out = func(input);
    let duration = start.elapsed();
    println!("Solution: {}", format!("{:?}", out).bold().green());
    println!("Duration: {}", format!("{:?}", duration).bold().blue());
    println!("");
}

pub fn run_day(year: &str, day: &str, contents: String) {
    println!(
        "=== {}, {} ===",
        format!("Year {}", year).bold(),
        format!("Day {}", day).bold()
    );
    println!("");

    let solution = get_solution(year, day);
    solution(contents);
}

pub fn run_viz(year: &str, day: &str, contents: String) {
    println!(
        "=== {}, {} ===",
        format!("Year {}", year).bold(),
        format!("Day {}", day).bold()
    );
    println!("");

    let visualization = match year {
        "2022" => match day {
            "09" => viz::year_2022_09::visualize,
            _ => panic!("No solution for this day"),
        },
        _ => panic!("Year not implemented"),
    };

    visualization(contents);
}

fn get_solution(year: &str, day: &str) -> fn(String) {
    match year {
        "2017" => match day {
            "11" => year_2017::day_11::solution,
            _ => panic!("No solution for this day"),
        },
        "2022" => match day {
            "01" => year_2022::day_01::solution,
            "02" => year_2022::day_02::solution,
            "03" => year_2022::day_03::solution,
            "04" => year_2022::day_04::solution,
            "05" => year_2022::day_05::solution,
            "06" => year_2022::day_06::solution,
            "07" => year_2022::day_07::solution,
            "08" => year_2022::day_08::solution,
            "09" => year_2022::day_09::solution,
            "10" => year_2022::day_10::solution,
            _ => panic!("No solution for this day"),
        },
        _ => panic!("Year not implemented"),
    }
}
