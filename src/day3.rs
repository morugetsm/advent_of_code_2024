pub fn part1() {
    let input = std::fs::read_to_string("./input/day3").unwrap();

    let rgx = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result = rgx.captures_iter(&input).fold(0u32, |acc, cpts| {
        let (_, [arg1, arg2]) = cpts.extract();

        acc + arg1.parse::<u32>().unwrap() * arg2.parse::<u32>().unwrap()
    });

    println!("Day3 Part1 result: {}", result);
}

pub fn part2() {
    let input = std::fs::read_to_string("./input/day3").unwrap();

    let rgx = regex::Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let result = rgx
        .captures_iter(&input)
        .fold((true, 0u32), |mut acc, cpts| {
            match cpts.get(0).unwrap().as_str() {
                "do()" => acc.0 = true,
                "don't()" => acc.0 = false,
                _ if acc.0 => {
                    let arg1 = cpts.get(1).unwrap().as_str();
                    let arg2 = cpts.get(2).unwrap().as_str();
                    acc.1 += arg1.parse::<u32>().unwrap() * arg2.parse::<u32>().unwrap();
                }
                _ => (),
            }
            acc
        });

    println!("Day3 Part2 result: {}", result.1);
}
