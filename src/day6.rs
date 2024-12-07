use std::{collections::HashSet, mem::MaybeUninit, ops::Range};

use advent_of_code_2024::Day;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Coord {
    y_range: Range<usize>,
    y: usize,
    x_range: Range<usize>,
    x: usize,
}

impl Coord {
    fn forward(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Up => {
                if self.y == self.y_range.start {
                    None
                } else {
                    Some(Self {
                        y: self.y - 1,
                        ..self.clone()
                    })
                }
            }
            Direction::Down => {
                if self.y == self.y_range.end - 1 {
                    None
                } else {
                    Some(Self {
                        y: self.y + 1,
                        ..self.clone()
                    })
                }
            }
            Direction::Left => {
                if self.x == self.x_range.start {
                    None
                } else {
                    Some(Self {
                        x: self.x - 1,
                        ..self.clone()
                    })
                }
            }
            Direction::Right => {
                if self.x == self.x_range.end - 1 {
                    None
                } else {
                    Some(Self {
                        x: self.x + 1,
                        ..self.clone()
                    })
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Coord2 {
    y: usize,
    y_max: usize,
    x: usize,
    x_max: usize,
}

impl Coord2 {
    fn forward(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::Up => (self.y > 0).then(|| Self {
                y: self.y - 1,
                ..*self
            }),
            Direction::Down => (self.y < self.y_max).then(|| Self {
                y: self.y + 1,
                ..*self
            }),
            Direction::Left => (self.x > 0).then(|| Self {
                x: self.x - 1,
                ..*self
            }),
            Direction::Right => (self.x < self.x_max).then(|| Self {
                x: self.x + 1,
                ..*self
            }),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Area {
    Empty,
    Block,
    Visited,
    Guard(Direction),
}

impl From<char> for Area {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Block,
            '^' => Self::Guard(Direction::Up),
            'v' => Self::Guard(Direction::Down),
            '<' => Self::Guard(Direction::Left),
            '>' => Self::Guard(Direction::Right),
            _ => panic!(),
        }
    }
}

pub struct Day6;

impl std::fmt::Display for Day6 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day6")
    }
}

impl Day for Day6 {
    fn part1(&self) -> usize {
        let input = std::fs::read_to_string("./input/day6").unwrap();

        let mut map: Vec<Vec<Area>> = input
            .lines()
            .map(|line| line.chars().map(Area::from).collect())
            .collect();

        let mut coord = MaybeUninit::uninit();

        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if matches!(map[y][x], Area::Guard(_)) {
                    coord.write(Coord {
                        y_range: 0..map.len(),
                        y,
                        x_range: 0..map[0].len(),
                        x,
                    });
                }
            }
        }

        let mut curr = unsafe { coord.assume_init() };

        'outer: loop {
            let mut guard = Area::Visited;
            std::mem::swap(&mut guard, &mut map[curr.y][curr.x]);

            let Area::Guard(ref mut dir) = guard else {
                panic!()
            };

            'inner: loop {
                let Some(next) = curr.forward(*dir) else {
                    break 'outer;
                };
                match map[next.y][next.x] {
                    Area::Block => {
                        dir.rotate();
                    }
                    Area::Empty | Area::Visited => {
                        std::mem::swap(&mut guard, &mut map[next.y][next.x]);
                        curr = next;
                        break 'inner;
                    }
                    _ => unreachable!(),
                }
            }
        }

        map.iter().fold(0, |acc, line| {
            acc + line.iter().fold(0, |acc, cell| {
                if matches!(cell, Area::Visited) {
                    acc + 1
                } else {
                    acc
                }
            })
        })
    }

    fn part2(&self) -> usize {
        let input = std::fs::read_to_string("./input/day6").unwrap();

        let map: Vec<Vec<Area>> = input
            .lines()
            .map(|line| line.chars().map(Area::from).collect())
            .collect();

        fn simulate_loop(map: Vec<Vec<Area>>) -> bool {
            let mut coord = MaybeUninit::uninit();

            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    if matches!(map[y][x], Area::Guard(_)) {
                        coord.write(Coord2 {
                            y,
                            y_max: map.len() - 1,
                            x,
                            x_max: map[0].len() - 1,
                        });
                    }
                }
            }

            let mut curr = unsafe { coord.assume_init() };
            let Area::Guard(mut dir) = map[curr.y][curr.x] else {
                panic!()
            };

            let mut path = HashSet::new();

            'outer: loop {
                'inner: loop {
                    let Some(next) = curr.forward(dir) else {
                        break 'outer;
                    };
                    if !path.insert((next.y, next.x, dir)) {
                        return true;
                    }
                    if map[next.y][next.x] == Area::Block {
                        dir.rotate();
                    } else {
                        curr = next;
                        break 'inner;
                    }
                }
            }

            false
        }

        let mut maps = Vec::new();

        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if !matches!(map[y][x], Area::Empty) {
                    continue;
                }
                let mut map = map.clone();
                map[y][x] = Area::Block;

                maps.push(map);
            }
        }

        maps.into_par_iter()
            .map(simulate_loop)
            .filter(|is_loop| *is_loop)
            .count()
    }
}
