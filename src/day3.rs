use advent_of_code_2024::Day;

pub struct Day3;

impl Day for Day3 {
    fn part1(&self) -> usize {
        let input = std::fs::read_to_string("./input/day3").unwrap();

        let rgx = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let result = rgx.captures_iter(&input).fold(0usize, |acc, cpts| {
            let (_, [arg1, arg2]) = cpts.extract();

            acc + arg1.parse::<usize>().unwrap() * arg2.parse::<usize>().unwrap()
        });

        result
    }

    fn part2(&self) -> usize {
        let input = std::fs::read_to_string("./input/day3").unwrap();

        let rgx = regex::Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

        let (_, result) = rgx
            .captures_iter(&input)
            .fold((true, 0usize), |mut acc, cpts| {
                match cpts.get(0).unwrap().as_str() {
                    "do()" => acc.0 = true,
                    "don't()" => acc.0 = false,
                    _ if acc.0 => {
                        let arg1 = cpts.get(1).unwrap().as_str();
                        let arg2 = cpts.get(2).unwrap().as_str();
                        acc.1 += arg1.parse::<usize>().unwrap() * arg2.parse::<usize>().unwrap();
                    }
                    _ => (),
                }
                acc
            });

        result
    }
}
