use advent_of_code_2024::Day;

pub struct Day7;

impl Day for Day7 {
    fn part1(&self) -> usize {
        let input = std::fs::read_to_string("./input/day7").unwrap();

        let rgx = regex::Regex::new(r"(\d+)").unwrap();

        let result: usize = input
            .lines()
            .map(|line| {
                let mut segs = rgx
                    .captures_iter(line)
                    .map(|cpt| cpt.get(1).unwrap().as_str().parse::<usize>().unwrap());
                (segs.next().unwrap(), segs.collect::<Vec<usize>>())
            })
            .filter_map(|(res, parts)| {
                let mut opps = vec![parts[0]];

                for part in &parts[1..] {
                    let bufs = std::mem::take(&mut opps);
                    for buf in bufs {
                        opps.push(buf + part);
                        opps.push(buf * part);
                    }
                }

                opps.contains(&res).then_some(res)
            })
            .sum();

        result
    }

    fn part2(&self) -> usize {
        let input = std::fs::read_to_string("./input/day7").unwrap();

        let rgx = regex::Regex::new(r"(\d+)").unwrap();

        let result: usize = input
            .lines()
            .map(|line| {
                let mut segs = rgx
                    .captures_iter(line)
                    .map(|cpt| cpt.get(1).unwrap().as_str().parse::<usize>().unwrap());
                (segs.next().unwrap(), segs.collect::<Vec<usize>>())
            })
            .filter_map(|(res, parts)| {
                let mut opps = vec![parts[0]];

                for part in &parts[1..] {
                    let bufs = std::mem::take(&mut opps);
                    for buf in bufs {
                        opps.push(buf + part);
                        opps.push(buf * part);
                        opps.push(format!("{buf}{part}").parse().unwrap());
                    }
                }

                opps.contains(&res).then_some(res)
            })
            .sum();

        result
    }
}
