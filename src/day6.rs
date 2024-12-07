use std::{collections::HashSet, mem::MaybeUninit, ops::Range};

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

#[derive(Clone, Debug)]
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

pub fn part1() {
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

    let result = map.iter().fold(0, |acc, line| {
        acc + line.iter().fold(0, |acc, cell| {
            if matches!(cell, Area::Visited) {
                acc + 1
            } else {
                acc
            }
        })
    });

    println!("Day6 Part1 result: {}", result);
}

pub fn part2() {
    let input = std::fs::read_to_string("./input/day6").unwrap();

    let map: Vec<Vec<Area>> = input
        .lines()
        .map(|line| line.chars().map(Area::from).collect())
        .collect();

    fn simulate_loop(mut map: Vec<Vec<Area>>) -> bool {
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
        let mut guard = Area::Visited;
        std::mem::swap(&mut guard, &mut map[curr.y][curr.x]);

        let mut path = HashSet::new();

        'outer: loop {
            let Area::Guard(ref mut dir) = guard else {
                panic!()
            };

            'inner: loop {
                let Some(next) = curr.forward(*dir) else {
                    break 'outer;
                };
                if !path.insert((next.y, next.x, *dir)) {
                    return true;
                }
                match map[next.y][next.x] {
                    Area::Block => {
                        dir.rotate();
                    }
                    Area::Empty | Area::Visited => {
                        curr = next;
                        break 'inner;
                    }
                    _ => unreachable!(),
                }
            }
        }

        false
    }

    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !matches!(map[y][x], Area::Empty) {
                continue;
            }
            let mut map = map.clone();
            map[y][x] = Area::Block;

            if simulate_loop(map) {
                result += 1;
            }
        }
    }

    println!("Day6 Part2 result: {}", result);
}
