pub fn run() {
    let input = include_str!("../../data/day_6.txt");
    let mut total: i128 = 0;

    let lines: Vec<&str> = input
        .lines()
        .map(str::trim_end)
        .filter(|l| !l.is_empty())
        .collect();

    let operations_line = lines.last().expect("file must have at least one line");
    let operations: Vec<char> = operations_line
        .split_whitespace()
        .map(|tok| tok.chars().next().unwrap())
        .collect();

    let data_lines = &lines[..lines.len() - 1];
    let grid: Vec<Vec<i128>> = data_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|tok| tok.parse::<i128>().expect("invalid number"))
                .collect()
        })
        .collect();

    for col in 0..grid[0].len() {
        let operation = operations[col];

        let mut value: i128 = match operation {
            '+' => 0,
            '*' => 1,
            _ => 0,
        };

        for row in 0..grid.len() {
            let val = grid[row][col];
            match operation {
                '+' => value += val,
                '*' => value *= val,
                _ => unreachable!(),
            }
        }

        total += value;
    }

    println!("Total result: {}", total);
}
