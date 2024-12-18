use std::collections::HashSet;

use advent_of_code_2024::Day;

pub struct Day10;

impl std::fmt::Display for Day10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day10")
    }
}

impl Day for Day10 {
    fn part1(&self) -> usize {
        let input = std::fs::read_to_string("./input/day10").unwrap();

        let map: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        fn find_path(map: &Vec<Vec<usize>>, x: usize, y: usize) -> HashSet<(usize, usize)> {
            let mut endpoints = HashSet::new();
            let level = map[y][x];

            if level == 9 {
                endpoints.insert((x, y));
                return endpoints;
            }

            if y > 0 && map[y - 1][x] == level + 1 {
                endpoints.extend(find_path(map, x, y - 1));
            }
            if y < map.len() - 1 && map[y + 1][x] == level + 1 {
                endpoints.extend(find_path(map, x, y + 1));
            }
            if x > 0 && map[y][x - 1] == level + 1 {
                endpoints.extend(find_path(map, x - 1, y));
            }
            if x < map[0].len() - 1 && map[y][x + 1] == level + 1 {
                endpoints.extend(find_path(map, x + 1, y));
            }

            endpoints
        }

        let mut result = 0;

        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == 0 {
                    result += find_path(&map, x, y).len();
                }
            }
        }

        result
    }

    fn part2(&self) -> usize {
        let input = std::fs::read_to_string("./input/day10").unwrap();

        let map: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        fn find_path_with_rating(map: &Vec<Vec<usize>>, x: usize, y: usize) -> usize {
            let level = map[y][x];

            if level == 9 {
                return 1;
            }

            let mut rating = 0;
            if y > 0 && map[y - 1][x] == level + 1 {
                rating += find_path_with_rating(map, x, y - 1);
            }
            if y < map.len() - 1 && map[y + 1][x] == level + 1 {
                rating += find_path_with_rating(map, x, y + 1);
            }
            if x > 0 && map[y][x - 1] == level + 1 {
                rating += find_path_with_rating(map, x - 1, y);
            }
            if x < map[0].len() - 1 && map[y][x + 1] == level + 1 {
                rating += find_path_with_rating(map, x + 1, y);
            }

            rating
        }

        let mut result = 0;

        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == 0 {
                    result += find_path_with_rating(&map, x, y);
                }
            }
        }

        result
    }
}
