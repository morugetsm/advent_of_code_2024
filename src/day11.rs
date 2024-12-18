use std::collections::HashMap;

use advent_of_code_2024::Day;

pub struct Day11;

impl std::fmt::Display for Day11 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day11")
    }
}

impl Day for Day11 {
    fn part1(&self) -> usize {
        let input = std::fs::read_to_string("./input/day11").unwrap();

        let stones: Vec<usize> = input
            .trim()
            .split(" ")
            .map(|seg| seg.parse().unwrap())
            .collect();

        let mut cache = HashMap::new();

        stones
            .into_iter()
            .map(|stone| splitted_to_after(&mut cache, stone, 25))
            .sum()
    }

    fn part2(&self) -> usize {
        let input = std::fs::read_to_string("./input/day11").unwrap();

        let stones: Vec<usize> = input
            .trim()
            .split(" ")
            .map(|seg| seg.parse().unwrap())
            .collect();

        let mut cache = HashMap::new();

        stones
            .into_iter()
            .map(|stone| splitted_to_after(&mut cache, stone, 75))
            .sum()
    }
}

// #[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
// struct Stone(usize);

// impl Stone {
//     fn next(mut self) -> (Stone, Option<Stone>) {
//         if self.0 == 0 {
//             self.0 += 1;
//             return (self, None);
//         }

//         let log = self.0.ilog10();
//         if log % 2 == 0 {
//             self.0 *= 2024;
//             return (self, None);
//         }

//         let splitter = 10usize.pow(log / 2 + 1);

//         let second = Self(self.0 % splitter);
//         self.0 /= splitter;

//         (self, Some(second))
//     }
// }

fn splitted_to_after(
    cache: &mut HashMap<(usize, usize), usize>,
    value: usize,
    blink: usize,
) -> usize {
    if blink == 0 {
        return 1;
    }
    if let Some(&splitted) = cache.get(&(value, blink)) {
        return splitted;
    }
    if value == 0 {
        let splitted = splitted_to_after(cache, 1, blink - 1);
        cache.insert((value, blink), splitted);
        return splitted;
    }

    let log = value.ilog10();
    if log % 2 == 0 {
        let splitted = splitted_to_after(cache, value * 2024, blink - 1);
        cache.insert((value, blink), splitted);
        return splitted;
    }

    let splitter = 10usize.pow(log / 2 + 1);
    let splitted = splitted_to_after(cache, value % splitter, blink - 1)
        + splitted_to_after(cache, value / splitter, blink - 1);
    cache.insert((value, blink), splitted);
    splitted
}
