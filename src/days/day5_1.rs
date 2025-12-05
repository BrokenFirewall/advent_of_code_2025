#[derive(Debug)]
struct Range {
    start: u128,
    end: u128,
}

impl Range {
    fn new(start: u128, end: u128) -> Self {
        Self { start, end }
    }

    fn contains(&self, value: u128) -> bool {
        value >= self.start && value <= self.end
    }
}

pub fn run() {
    let input = include_str!("../../data/day_5.txt");

    let mut ranges: Vec<Range> = Vec::new();
    let mut values: Vec<u128> = Vec::new();
    let mut total: i32 = 0;

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some((start, end)) = line.split_once('-') {
            let start = start.parse::<u128>().unwrap();
            let end = end.parse::<u128>().unwrap();
            ranges.push(Range::new(start, end));
        } else {
            let single = line.parse::<u128>().unwrap();
            values.push(single);
        }
    }

    for value in values {
        for range in &ranges {
            if range.contains(value) {
                total += 1;
                break;
            }
        }
    }

    println!("total: {}", total);
}
