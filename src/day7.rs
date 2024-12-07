pub fn part1() {
    let input = std::fs::read_to_string("./input/day7").unwrap();

    let rgx = regex::Regex::new(r"(\d+)").unwrap();

    let result: u64 = input
        .lines()
        .map(|line| {
            let mut segs = rgx
                .captures_iter(line)
                .map(|cpt| cpt.get(1).unwrap().as_str().parse::<u64>().unwrap());
            (segs.next().unwrap(), segs.collect::<Vec<u64>>())
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

    println!("Day7 Part1 result: {}", result);
}

pub fn part2() {
    let input = std::fs::read_to_string("./input/day7").unwrap();

    let rgx = regex::Regex::new(r"(\d+)").unwrap();

    let result: u64 = input
        .lines()
        .map(|line| {
            let mut segs = rgx
                .captures_iter(line)
                .map(|cpt| cpt.get(1).unwrap().as_str().parse::<u64>().unwrap());
            (segs.next().unwrap(), segs.collect::<Vec<u64>>())
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

    println!("Day7 Part2 result: {}", result);
}
