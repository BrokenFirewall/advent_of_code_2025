fn calculate(position: i32, direction: &str, steps: i32) -> i32 {
    let mut value = position;

    match direction {
        "R" => {
            value = (value + steps) % 100;
        }
        "L" => {
            value = (value + 100 - steps) % 100;
        }
        _ => panic!("Invalid direction"),
    }

    return value;
}

pub fn run() {
    let input = include_str!("../../data/day_1.txt");

    let mut position = 50;
    let mut zero_counter = 0;

    for line in input.lines() {
        let direction = &line[0..1];
        let steps: i32 = line[1..].parse().unwrap();

        position = calculate(position, direction, steps);

        if position == 0 {
            zero_counter += 1;
        }
    }

    println!("Day 1 solution: {}", zero_counter);
}
