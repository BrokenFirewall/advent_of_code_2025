fn calculate_max_joltage(row: &str) -> u32 {
    let mut max_value: u32 = 0;

    let chars: Vec<u32> = row.chars().map(|c: char| c.to_digit(10).unwrap()).collect();

    for i in 0..chars.len() {
        let value: u32 = chars[i];

        for j in i + 1..chars.len() {
            let other = chars[j];

            let combined = format!("{}{}", value, other);

            let complete_value: u32 = combined.parse().unwrap();

            if complete_value > max_value {
                max_value = complete_value;
            }
        }
    }

    max_value
}

pub fn run() {
    let input = include_str!("../../data/day_3.txt");

    let mut total_joltage: u32 = 0;

    for row in input.lines() {
        total_joltage += calculate_max_joltage(row);
    }

    println!("Solution: {}", total_joltage);
}
