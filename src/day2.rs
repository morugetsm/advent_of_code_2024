pub fn part1() {
    let input = std::fs::read_to_string("./input/day2").unwrap();

    let result = input
        .lines()
        .map(|line| line.split(" ").map(|seg| seg.parse().unwrap()).collect())
        .fold(0, |acc, mut segs: Vec<u8>| {
            if !segs.is_sorted() {
                segs.reverse();
                if !segs.is_sorted() {
                    return acc;
                }
            }
            if !segs.windows(2).all(|vals| {
                let [val1, val2] = vals else {
                    return false;
                };
                (0..=3).contains(&val1.abs_diff(*val2))
            }) {
                return acc;
            }
            acc + 1
        });

    println!("Day2 Part1 result: {}", result);
}

pub fn part2() {
    let input = std::fs::read_to_string("./input/day2").unwrap();

    #[derive(Default, Debug)]
    struct Dampener {
        ascend: u8,
        descend: u8,
        danger: u8,
    }

    let result = input
        .lines()
        .map(|line| line.split(" ").map(|seg| seg.parse().unwrap()).collect())
        .fold(0, |acc, segs: Vec<u8>| {
            fn is_safe(segs: Vec<u8>) -> bool {
                let mut damp = Dampener::default();

                for vals in segs.windows(2) {
                    let [val1, val2] = vals else {
                        continue;
                    };
                    match val2.cmp(val1) {
                        std::cmp::Ordering::Greater if (1..=3).contains(&val1.abs_diff(*val2)) => {
                            damp.ascend += 1
                        }
                        std::cmp::Ordering::Less if (1..=3).contains(&val1.abs_diff(*val2)) => {
                            damp.descend += 1
                        }
                        _ => damp.danger += 1,
                    }
                }

                damp.ascend.min(damp.descend) + damp.danger == 0
            }

            if is_safe(segs.clone()) {
                return acc + 1;
            }
            for i in 0..segs.len() {
                let mut segs = segs.clone();
                segs.remove(i);

                if is_safe(segs) {
                    return acc + 1;
                }
            }

            acc
        });

    println!("Day2 Part1 result: {}", result);
}
