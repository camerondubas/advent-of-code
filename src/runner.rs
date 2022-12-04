use colored::Colorize;
use std::{fmt::Debug, time::Instant};

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
