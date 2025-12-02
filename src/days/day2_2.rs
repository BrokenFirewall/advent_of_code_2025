fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    for l in 1..=len / 2 {
        if len % l != 0 {
            continue;
        }

        let pattern = &s[..l];
        let repeated = pattern.repeat(len / l);

        if repeated == s {
            return true;
        }
    }

    false
}

fn count_invalid_ids(first: u64, last: u64) -> u64 {
    let mut invalid = 0;

    for n in first..=last {
        if is_invalid_id(n) {
            invalid += n;
        }
    }

    invalid
}

pub fn run() {
    let input = include_str!("../../data/day_2.txt");

    let mut invalid_ids = 0;

    for id in input.trim().split(',') {
        let mut parts = id.split('-');

        let first_id = parts.next().unwrap().parse::<u64>().unwrap();
        let second_id = parts.next().unwrap().parse::<u64>().unwrap();

        invalid_ids += count_invalid_ids(first_id, second_id)
    }

    println!("Solution: {}", invalid_ids);
}
