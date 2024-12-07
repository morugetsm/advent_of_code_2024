use std::collections::HashMap;

use advent_of_code_2024::Day;

pub struct Day1;

impl Day for Day1 {
    fn part1(&self) -> usize {
        let input = std::fs::read_to_string("./input/day1").unwrap();

        let extract_pair = regex::Regex::new(r"^(\d*)\s*(\d*)$").unwrap();

        let (mut list1, mut list2) =
            input
                .lines()
                .fold((Vec::new(), Vec::new()), |(mut list1, mut list2), line| {
                    let (_, [seg1, seg2]) = extract_pair.captures(line).unwrap().extract();

                    list1.push(seg1.parse::<usize>().unwrap());
                    list2.push(seg2.parse::<usize>().unwrap());

                    (list1, list2)
                });

        list1.sort();
        list2.sort();

        let result = list1
            .into_iter()
            .zip(list2)
            .fold(0, |acc, (val1, val2)| acc + val1.abs_diff(val2));

        result
    }

    fn part2(&self) -> usize {
        let input = std::fs::read_to_string("./input/day1").unwrap();

        let extract_pair = regex::Regex::new(r"^(\d*)\s*(\d*)$").unwrap();

        let (list1, list2) =
            input
                .lines()
                .fold((Vec::new(), Vec::new()), |(mut list1, mut list2), line| {
                    let (_, [seg1, seg2]) = extract_pair.captures(line).unwrap().extract();

                    list1.push(seg1.parse::<usize>().unwrap());
                    list2.push(seg2.parse::<usize>().unwrap());

                    (list1, list2)
                });

        let scores = list2.into_iter().fold(HashMap::new(), |mut acc, val| {
            if let Some(score) = acc.get_mut(&val) {
                *score += 1;
            } else {
                acc.insert(val, 1);
            }
            acc
        });

        let result = list1.into_iter().fold(0, |acc, val| {
            let score = scores.get(&val).copied().unwrap_or(0);
            acc + val * score
        });

        result
    }
}
