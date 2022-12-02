use colored::Colorize;
use std::time::Instant;

pub fn run(name: &str, func: fn(&String) -> u32, input: &String) -> () {
    println!("=== {} ===", format!("{}", name).bold());
    let start = Instant::now();
    let out = func(input);
    let duration = start.elapsed();
    println!("Solution: {}", format!("{}", out).bold().green());
    println!("Duration: {}", format!("{:?}", duration).bold().blue());
    println!("");
}
