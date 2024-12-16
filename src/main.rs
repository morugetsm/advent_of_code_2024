use advent_of_code_2024::Day;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    let days: &[Box<dyn Day>] = &[
        Box::new(day1::Day1),
        Box::new(day2::Day2),
        Box::new(day3::Day3),
        Box::new(day4::Day4),
        Box::new(day5::Day5),
        Box::new(day6::Day6),
        Box::new(day7::Day7),
        Box::new(day8::Day8),
        Box::new(day9::Day9),
    ];

    println!("ADVENT OF CODE 2024!");

    for day in days.iter() {
        println!();

        let timer1 = std::time::Instant::now();
        let result1 = day.part1();
        let elapse1 = timer1.elapsed();
        println!(
            "[{} Part1] {:<15} ({}ms)",
            day.as_ref(),
            result1,
            elapse1.as_millis()
        );

        let timer2 = std::time::Instant::now();
        let result2 = day.part2();
        let elapse2 = timer2.elapsed();
        println!(
            "[{} Part2] {:<15} ({}ms)",
            day.as_ref(),
            result2,
            elapse2.as_millis()
        );
    }
}
