mod days {
    pub mod day1_1;
    pub mod day1_2;
    pub mod day2_1;
    pub mod day2_2;
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
        _ => eprintln!("Day not implemented"),
    }
}
