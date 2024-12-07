use std::collections::HashMap;

pub fn part1() {
    let input = std::fs::read_to_string("./input/day5").unwrap();

    let (after_rules, before_rules): (HashMap<u32, Vec<u32>>, HashMap<u32, Vec<u32>>) = input
        .lines()
        .take_while(|line| !line.is_empty())
        .fold(Default::default(), |(mut after, mut before), line| {
            let segs: Vec<&str> = line.split('|').collect();

            let val1 = segs[0].parse::<u32>().unwrap();
            let val2 = segs[1].parse::<u32>().unwrap();

            if let Some(store) = after.get_mut(&val1) {
                store.push(val2);
            } else {
                after.insert(val1, vec![val2]);
            }
            if let Some(store) = before.get_mut(&val2) {
                store.push(val1);
            } else {
                before.insert(val2, vec![val1]);
            }

            (after, before)
        });

    let result: u32 = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .filter_map(|line| {
            let mut segs: Vec<u32> = line.split(",").map(|seg| seg.parse().unwrap()).collect();

            for (idx, seg) in segs.iter().enumerate() {
                if let Some(after_rule) = after_rules.get(seg) {
                    for before in &segs[..idx] {
                        if after_rule.contains(before) {
                            return None;
                        }
                    }
                }
                if let Some(before_rule) = before_rules.get(seg) {
                    for after in &segs[(idx + 1)..] {
                        if before_rule.contains(after) {
                            return None;
                        }
                    }
                }
            }

            Some(segs.remove(segs.len() / 2))
        })
        .sum();

    println!("Day5 Part1 result: {}", result);
}

pub fn part2() {
    let input = std::fs::read_to_string("./input/day5").unwrap();

    let (after_rules, before_rules): (HashMap<u32, Vec<u32>>, HashMap<u32, Vec<u32>>) = input
        .lines()
        .take_while(|line| !line.is_empty())
        .fold(Default::default(), |(mut after, mut before), line| {
            let segs: Vec<&str> = line.split('|').collect();

            let val1 = segs[0].parse::<u32>().unwrap();
            let val2 = segs[1].parse::<u32>().unwrap();

            if let Some(store) = after.get_mut(&val1) {
                store.push(val2);
            } else {
                after.insert(val1, vec![val2]);
            }
            if let Some(store) = before.get_mut(&val2) {
                store.push(val1);
            } else {
                before.insert(val2, vec![val1]);
            }

            (after, before)
        });

    let result: u32 = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .filter_map(|line| {
            let segs: Vec<u32> = line.split(",").map(|seg| seg.parse().unwrap()).collect();

            for (idx, seg) in segs.iter().enumerate() {
                if let Some(after_rule) = after_rules.get(seg) {
                    for before in &segs[..idx] {
                        if after_rule.contains(before) {
                            return Some(segs);
                        }
                    }
                }
                if let Some(before_rule) = before_rules.get(seg) {
                    for after in &segs[(idx + 1)..] {
                        if before_rule.contains(after) {
                            return Some(segs);
                        }
                    }
                }
            }

            None
        })
        .map(|mut segs| {
            segs.sort_by(|seg1, seg2| {
                if after_rules
                    .get(seg1)
                    .map(|rule| rule.contains(seg2))
                    .unwrap_or(false)
                {
                    return std::cmp::Ordering::Less;
                }
                if before_rules
                    .get(seg1)
                    .map(|rule| rule.contains(seg2))
                    .unwrap_or(false)
                {
                    return std::cmp::Ordering::Greater;
                }
                if after_rules
                    .get(seg2)
                    .map(|rule| rule.contains(seg1))
                    .unwrap_or(false)
                {
                    return std::cmp::Ordering::Greater;
                }
                if before_rules
                    .get(seg2)
                    .map(|rule| rule.contains(seg1))
                    .unwrap_or(false)
                {
                    return std::cmp::Ordering::Less;
                }
                std::cmp::Ordering::Equal
            });

            segs.remove(segs.len() / 2)
        })
        .sum();

    println!("Day5 Part2 result: {}", result);
}
