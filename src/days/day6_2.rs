pub fn run() {
    let input = include_str!("../../data/day_6.txt");

    let lines: Vec<String> = input
        .lines()
        .map(|l| l.to_string())
        .filter(|l| !l.trim_end().is_empty())
        .collect();

    let operations_line = lines.last().expect("file must have at least one line");
    let operations: Vec<char> = operations_line
        .split_whitespace()
        .map(|tok| tok.chars().next().unwrap())
        .collect();

    let data_lines = &lines[..lines.len() - 1];
    let width = lines.iter().map(|l| l.len()).max().unwrap();

    let grid: Vec<Vec<char>> = data_lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(width, ' ');
            chars
        })
        .collect();

    let height = grid.len();

    let mut rotated = Vec::new();
    for x in (0..width).rev() {
        let mut new_row = Vec::with_capacity(height);
        for y in 0..height {
            new_row.push(grid[y][x]);
        }
        rotated.push(new_row);
    }

    let mut results: Vec<i128> = Vec::new();
    let mut current_numbers: Vec<i128> = Vec::new();
    let mut op_index = operations.len();

    let mut finalize_group = |nums: &mut Vec<i128>, results: &mut Vec<i128>| {
        if nums.is_empty() {
            return;
        }

        op_index -= 1;

        let op = operations[op_index];
        let result = match op {
            '+' => nums.iter().copied().sum::<i128>(),
            '*' => nums.iter().copied().product::<i128>(),
            _ => panic!("Unknown operator",),
        };

        results.push(result);
        nums.clear();
    };

    for row in &rotated {
        let is_empty = row.iter().all(|c| *c == ' ');

        if is_empty {
            finalize_group(&mut current_numbers, &mut results);
        } else {
            let digits: String = row.iter().filter(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() {
                current_numbers.push(digits.parse().unwrap());
            }
        }
    }

    finalize_group(&mut current_numbers, &mut results);

    let total: i128 = results.iter().copied().sum();
    println!("Total: {total}");
}
