#[derive(Debug, Clone, PartialEq)]
struct Boxes {
    x: i64,
    y: i64,
    z: i64,
}

fn calculate_distance(a: &Boxes, b: &Boxes) -> f64 {
    let dx = (a.x - b.x).pow(2);
    let dy = (a.y - b.y).pow(2);
    let dz = (a.z - b.z).pow(2);

    ((dx + dy + dz) as f64).sqrt()
}

pub fn run() {
    let input = include_str!("../../data/day_8.txt");

    let boxes: Vec<Boxes> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut p = line.split(',');
            Boxes {
                x: p.next().unwrap().parse().unwrap(),
                y: p.next().unwrap().parse().unwrap(),
                z: p.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    let mut distances = Vec::new();

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            distances.push((
                calculate_distance(&boxes[i], &boxes[j]),
                &boxes[i],
                &boxes[j],
            ));
        }
    }

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut new_boxes: Vec<Vec<Boxes>> = boxes.iter().cloned().map(|b| vec![b]).collect();
    let mut response: i64 = 0;

    for &(_, a, b) in distances.iter() {
        let mut group_a = None;
        let mut group_b = None;

        for (idx, group) in new_boxes.iter().enumerate() {
            if group
                .iter()
                .any(|bx| bx.x == a.x && bx.y == a.y && bx.z == a.z)
            {
                group_a = Some(idx);
            }
            if group
                .iter()
                .any(|bx| bx.x == b.x && bx.y == b.y && bx.z == b.z)
            {
                group_b = Some(idx);
            }
        }

        // If either wasn't found (shouldn't happen), skip
        let (Some(ga), Some(gb)) = (group_a, group_b) else {
            continue;
        };

        // If they are already in the same circuit, this connection does nothing
        if ga == gb {
            continue;
        }

        // Merge the two circuits: always remove the higher index to avoid shifting issues
        let (keep, remove) = if ga < gb { (ga, gb) } else { (gb, ga) };

        let mut extracted = new_boxes.remove(remove);
        new_boxes[keep].append(&mut extracted);

        if new_boxes.len() == 1 {
            response = a.x * b.x;
            break;
        }
    }

    println!("response: {:?}", response);
}
