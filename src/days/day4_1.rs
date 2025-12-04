fn count_at_neighbors(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;

    let x = x as isize;
    let y = y as isize;

    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && nx < width && ny >= 0 && ny < height {
                if grid[ny as usize][nx as usize] == b'@' {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn run() {
    let input = include_str!("../../data/day_4.txt");
    let mut total: u32 = 0;

    let grid: Vec<Vec<u8>> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.as_bytes().to_vec())
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == b'@' {
                let count = count_at_neighbors(&grid, x, y);

                if count < 4 {
                    total += 1;
                }
            }
        }
    }

    println!("total: {}", total);
}
