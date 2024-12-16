use advent_of_code_2024::Day;

pub struct Day9;

impl std::fmt::Display for Day9 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day9")
    }
}

impl Day for Day9 {
    fn part1(&self) -> usize {
        let input = std::fs::read_to_string("./input/day9").unwrap();

        let mut id = 0;

        let mut disk: Vec<Option<usize>> = input
            .trim()
            .chars()
            .enumerate()
            .flat_map(|(idx, c)| {
                let n = c.to_digit(10).unwrap() as usize;
                let mut seg = Vec::with_capacity(n);
                let val = if idx % 2 == 0 {
                    let v = id;
                    id += 1;
                    Some(v)
                } else {
                    None
                };
                seg.resize(n, val);
                seg
            })
            .collect();

        'outer: for i in 0..disk.len() {
            let back = disk.len() - i - 1;
            if disk[back].is_none() {
                continue;
            }

            'inner: for front in 0..disk.len() {
                if back == front {
                    break 'outer;
                }
                if disk[front].is_some() {
                    continue;
                }
                disk.swap(back, front);
                break 'inner;
            }
        }

        disk.into_iter().enumerate().fold(0, |acc, (idx, num)| {
            acc + if let Some(n) = num { idx * n } else { 0 }
        })
    }

    fn part2(&self) -> usize {
        let input = std::fs::read_to_string("./input/day9").unwrap();

        let mut id = 0;

        let mut disk: Vec<Option<usize>> = input
            .trim()
            .chars()
            .enumerate()
            .flat_map(|(idx, c)| {
                let n = c.to_digit(10).unwrap() as usize;
                let mut seg = Vec::with_capacity(n);
                let val = if idx % 2 == 0 {
                    let v = id;
                    id += 1;
                    Some(v)
                } else {
                    None
                };
                seg.resize(n, val);
                seg
            })
            .collect();

        let mut i = disk.len() - 1;
        loop {
            if i == 0 {
                break;
            }
            let back = i;
            if disk[back].is_none() {
                i -= 1;
                continue;
            }
            
            let mut len = 0;
            for _ in 0.. {
                if back - len == 0 {
                    break;
                }
                len += 1;
                if disk[back - len] != disk[back] {
                    break;
                }
            }
            i -= len;

            'inner: for front in 0..disk.len() {
                if back == front {
                    break 'inner;
                }
                for j in 0..len {
                    if disk[front + j].is_some() {
                        continue 'inner;
                    }
                }
                for j in 0..len {
                    disk.swap(back - j , front + j);
                }
                break 'inner;
            }
        }

        disk.into_iter().enumerate().fold(0, |acc, (idx, num)| {
            acc + if let Some(n) = num { idx * n } else { 0 }
        })
    }
}
