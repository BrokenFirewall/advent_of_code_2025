mod days {
    pub mod day1_1;
    pub mod day1_2;
    pub mod day2_1;
    pub mod day2_2;
    pub mod day3_1;
    pub mod day3_2;
    pub mod day4_1;
    pub mod day4_2;
    pub mod day5_1;
    pub mod day5_2;
    pub mod day6_1;
    pub mod day6_2;
    pub mod day8_1;
    pub mod day8_2;
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <day>");
        return;
    }

    match args[1].as_str() {
        "11" => days::day1_1::run(),
        "12" => days::day1_2::run(),
        "21" => days::day2_1::run(),
        "22" => days::day2_2::run(),
        "31" => days::day3_1::run(),
        "32" => days::day3_2::run(),
        "41" => days::day4_1::run(),
        "42" => days::day4_2::run(),
        "51" => days::day5_1::run(),
        "52" => days::day5_2::run(),
        "61" => days::day6_1::run(),
        "62" => days::day6_2::run(),
        "81" => days::day8_1::run(),
        "82" => days::day8_2::run(),
        _ => eprintln!("Day not implemented"),
    }
}
