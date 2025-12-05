#[derive(Debug, Clone)]
struct Range {
    start: u128,
    end: u128,
}

impl Range {
    fn new(start: u128, end: u128) -> Self {
        Self { start, end }
    }
}

fn count_unique_ranges(mut ranges: Vec<Range>) -> u128 {
    ranges.sort_by_key(|r| r.start);

    let mut merged: Vec<Range> = Vec::new();
    let mut current = ranges[0].clone();

    for range in ranges.into_iter().skip(1) {
        if range.start <= current.end + 1 {
            current.end = current.end.max(range.end);
        } else {
            merged.push(current);
            current = range;
        }
    }

    merged.push(current);

    merged.into_iter().map(|r| r.end - r.start + 1).sum()
}

pub fn run() {
    let input = include_str!("../../data/day_5.txt");

    let mut ranges: Vec<Range> = Vec::new();

    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some((start, end)) = line.split_once('-') {
            let start = start.parse::<u128>().unwrap();
            let end = end.parse::<u128>().unwrap();
            ranges.push(Range::new(start, end));
        }
    }

    let total = count_unique_ranges(ranges);

    println!("total: {}", total);
}
