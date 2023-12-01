use std::fs;

pub mod runner;
pub mod viz;
pub mod year_2017;
pub mod year_2022;
pub mod year_2023;

pub fn get_input(year: &str, day: &str, input: Option<&str>) -> String {
    let input = match input {
        Some(input) => format!("input-{}", input),
        None => "input".to_string(),
    };
    let path = format!("./{}/{}/{}.txt", year, day, input);
    fs::read_to_string(path).expect("Should have been able to read the file")
}
