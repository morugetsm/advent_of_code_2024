use advent_of_code_2024::Day;

pub struct Day7;

impl Day for Day7 {
    fn part1(&self) -> usize {
        let input = std::fs::read_to_string("./input/day7").unwrap();

        let rgx = regex::Regex::new(r"(\d+)").unwrap();

        let result: usize = input
            .lines()
            .map(|line| {
                let mut nums = rgx
                    .captures_iter(line)
                    .map(|cpt| cpt.get(1).unwrap().as_str().parse::<usize>().unwrap());
                (nums.next().unwrap(), nums.collect::<Vec<usize>>())
            })
            .filter_map(|(res, operands)| {
                let mut opps = vec![operands[0]];

                for operand in &operands[1..] {
                    let mids = std::mem::take(&mut opps);
                    for mid in mids {
                        opps.push(mid + operand);
                        opps.push(mid * operand);
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
                let mut nums = rgx
                    .captures_iter(line)
                    .map(|cpt| cpt.get(1).unwrap().as_str().parse::<usize>().unwrap());
                (nums.next().unwrap(), nums.collect::<Vec<usize>>())
            })
            .filter_map(|(res, operands)| {
                let mut opps = vec![operands[0]];

                for operand in &operands[1..] {
                    let mids = std::mem::take(&mut opps);
                    for mid in mids {
                        opps.push(mid + operand);
                        opps.push(mid * operand);
                        opps.push(mid * 10usize.pow(operand.ilog10() + 1) + operand);
                    }
                }

                opps.contains(&res).then_some(res)
            })
            .sum();

        result
    }
}
