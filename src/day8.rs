use std::collections::HashSet;

use advent_of_code_2024::Day;

pub struct Day8;

impl std::fmt::Display for Day8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day8")
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
enum Area {
    #[default]
    Empty,
    Antenna(char),
}

impl From<char> for Area {
    fn from(value: char) -> Self {
        if value == '.' || value == '#' {
            Self::Empty
        } else {
            Self::Antenna(value)
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Coord {
    x: usize,
    x_limit: usize,
    y: usize,
    y_limit: usize,
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Coord {
    fn calc_antinodes(&self, other: Point) -> (Option<Point>, Option<Point>) {
        let x_diff = self.x.abs_diff(other.x);
        let y_diff = self.y.abs_diff(other.y);

        let an1 = 'an: {
            let x_may = if self.x < other.x {
                self.x.checked_sub(x_diff)
            } else {
                Some(self.x + x_diff).filter(|x| *x < self.x_limit)
            };
            let Some(x) = x_may else {
                break 'an None;
            };

            let y_may = if self.y < other.y {
                self.y.checked_sub(y_diff)
            } else {
                Some(self.y + y_diff).filter(|y| *y < self.y_limit)
            };
            let Some(y) = y_may else {
                break 'an None;
            };

            Some(Point { x, y })
        };

        let an2 = 'an: {
            let x_may = if other.x < self.x {
                other.x.checked_sub(x_diff)
            } else {
                Some(other.x + x_diff).filter(|x| *x < self.x_limit)
            };
            let Some(x) = x_may else {
                break 'an None;
            };

            let y_may = if other.y < self.y {
                other.y.checked_sub(y_diff)
            } else {
                Some(other.y + y_diff).filter(|y| *y < self.y_limit)
            };
            let Some(y) = y_may else {
                break 'an None;
            };

            Some(Point { x, y })
        };

        (an1, an2)
    }

    fn calc_antinodes_all(&self, other: Point) -> Vec<Point> {
        let mut antinodes = Vec::new();

        let x_difference = self.x.abs_diff(other.x);
        let y_difference = self.y.abs_diff(other.y);

        if x_difference == 0 && y_difference == 0 {
            return antinodes;
        }

        for i in 0.. {
            let x_diff = x_difference * i;
            let x_may = if self.x < other.x {
                self.x.checked_sub(x_diff)
            } else {
                Some(self.x + x_diff).filter(|x| *x < self.x_limit)
            };
            let Some(x) = x_may else {
                break;
            };

            let y_diff = y_difference * i;
            let y_may = if self.y < other.y {
                self.y.checked_sub(y_diff)
            } else {
                Some(self.y + y_diff).filter(|y| *y < self.y_limit)
            };
            let Some(y) = y_may else {
                break;
            };

            antinodes.push(Point { x, y })
        }

        for i in 0.. {
            let x_diff = x_difference * i;
            let x_may = if other.x < self.x {
                other.x.checked_sub(x_diff)
            } else {
                Some(other.x + x_diff).filter(|x| *x < self.x_limit)
            };
            let Some(x) = x_may else {
                break;
            };

            let y_diff = y_difference * i;
            let y_may = if other.y < self.y {
                other.y.checked_sub(y_diff)
            } else {
                Some(other.y + y_diff).filter(|y| *y < self.y_limit)
            };
            let Some(y) = y_may else {
                break;
            };

            antinodes.push(Point { x, y });
        }

        antinodes
    }
}

impl Day for Day8 {
    fn part1(&self) -> usize {
        let input = std::fs::read_to_string("./input/day8").unwrap();

        let map: Vec<Vec<Area>> = input
            .lines()
            .map(|line| line.chars().map(Area::from).collect())
            .collect();

        let antennas: Vec<(char, Coord)> = map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(x, area)| {
                        if let Area::Antenna(c) = area {
                            Some((
                                *c,
                                Coord {
                                    x,
                                    x_limit: map[0].len(),
                                    y,
                                    y_limit: map.len(),
                                },
                            ))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(char, Coord)>>()
            })
            .collect();

        let mut antinodes = HashSet::new();

        for (an_code, an_coord) in antennas {
            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    if x == an_coord.x && y == an_coord.y {
                        continue;
                    }
                    match map[y][x] {
                        Area::Antenna(c) if c == an_code => {
                            let (an1, an2) = an_coord.calc_antinodes(Point { x, y });
                            if let Some(an) = an1 {
                                antinodes.insert(an);
                            }
                            if let Some(an) = an2 {
                                antinodes.insert(an);
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        antinodes.len()
    }

    fn part2(&self) -> usize {
        let input = std::fs::read_to_string("./input/day8").unwrap();

        let map: Vec<Vec<Area>> = input
            .lines()
            .map(|line| line.chars().map(Area::from).collect())
            .collect();

        let antennas: Vec<(char, Coord)> = map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(x, area)| {
                        if let Area::Antenna(c) = area {
                            Some((
                                *c,
                                Coord {
                                    x,
                                    x_limit: map[0].len(),
                                    y,
                                    y_limit: map.len(),
                                },
                            ))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(char, Coord)>>()
            })
            .collect();

        let mut antinodes: HashSet<Point> = HashSet::new();

        for (an_code, an_coord) in antennas {
            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    if x == an_coord.x && y == an_coord.y {
                        continue;
                    }
                    match map[y][x] {
                        Area::Antenna(c) if c == an_code => {
                            let ans = an_coord.calc_antinodes_all(Point { x, y });
                            antinodes.extend(ans.into_iter());
                        }
                        _ => (),
                    }
                }
            }
        }

        antinodes.len()
    }
}
