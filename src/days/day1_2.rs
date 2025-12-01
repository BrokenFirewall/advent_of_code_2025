fn calculate_and_count_zeroes(position: i32, direction: &str, steps: i32) -> (i32, i32) {
    let mut value = position;
    let mut zero_hits = 0;

    for _ in 0..steps {
        if direction == "R" {
            value += 1;
            if value > 99 {
                value = 0;
            }
        } else if direction == "L" {
            value -= 1;
            if value < 0 {
                value = 99;
            }
        }

        if value == 0 {
            zero_hits += 1;
        }
    }

    (value, zero_hits)
}

pub fn run() {
    let input = include_str!("../../data/day_1.txt");

    let mut position = 50;
    let mut zero_counter = 0;

    for line in input.lines() {
        let direction = &line[0..1];
        let steps: i32 = line[1..].parse().unwrap();

        let (new_position, zero_hits) = calculate_and_count_zeroes(position, direction, steps);

        zero_counter += zero_hits;
        position = new_position;
    }

    println!("Day 1 solution: {}", zero_counter);
}
