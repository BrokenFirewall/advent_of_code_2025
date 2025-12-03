fn calculate_max_joltage(row: &str) -> u128 {
    let mut max_value = String::from("");

    let chars: Vec<u32> = row.chars().map(|c: char| c.to_digit(10).unwrap()).collect();

    let mut summed_index = 0;

    for i in (0..=11).rev() {
        let left_array: Vec<u32> = chars[summed_index..chars.len() - i].to_vec();

        let mut hightest_value = 0;
        let mut hightest_index = 0;

        for (j, val) in left_array.into_iter().enumerate() {
            if val > hightest_value {
                hightest_value = val;
                hightest_index = j + 1;
            }
        }

        summed_index += hightest_index;
        max_value = format!("{}{}", max_value, hightest_value);
    }

    return max_value.parse().unwrap();
}

pub fn run() {
    let input = include_str!("../../data/day_3.txt");

    let mut total_joltage: u128 = 0;

    for row in input.lines() {
        total_joltage += calculate_max_joltage(row);
    }

    println!("Solution: {}", total_joltage);
}
