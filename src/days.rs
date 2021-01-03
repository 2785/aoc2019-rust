pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use clap::Clap;
use std::fs;

#[derive(Clap)]
pub struct Day1 {
    #[clap(short = '1', long = "d1", default_value = "inputs/d1")]
    input: String,
}

impl Day1 {
    pub fn run(&self) {
        let f = fs::read_to_string(&self.input).expect("error reading file");
        let input = {
            match day1::parse_input(String::from(f)) {
                Ok(v) => v,
                Err(e) => panic!(e),
            }
        };
        print!("Part 1 Solution: {}\n", day1::solve_part1(&input));
        print!("Part 2 Solution: {}\n", day1::solve_part2(&input));
    }
}

#[derive(Clap)]
pub struct Day2 {
    #[clap(short = '2', long = "d2", default_value = "inputs/d2")]
    input: String,
}

impl Day2 {
    pub fn run(&self) {
        let f = fs::read_to_string(&self.input).expect("error reading file");
        let com = day2::parse_input(f).expect("error parsing input");

        print!(
            "Part 1 Solution: {}\n",
            day2::solve_part_1(&mut com.clone()).expect("error solving part 1")
        );

        print!(
            "Part 2 Solution: {}\n",
            day2::solve_part_2(&mut com.clone()).expect("error solving part 2")
        );
    }
}

#[derive(Clap)]
pub struct Day3 {
    #[clap(short = '3', long = "d3", default_value = "inputs/d3")]
    input: String,
}

impl Day3 {
    pub fn run(&self) {
        let f = fs::read_to_string(&self.input).expect("error reading file");
        let input = day3::parse_input(f).expect("error parsing input");

        print!(
            "Part 1 Solution: {}\n",
            day3::solve_part_1(&input.0, &input.1)
        );

        print!(
            "Part 2 Solution: {}\n",
            day3::solve_part_2(&input.0, &input.1)
        );
    }
}

#[derive(Clap)]
pub struct Day4 {
    #[clap(short = '4', long = "d4", default_value = "inputs/d4")]
    input: String,
}

impl Day4 {
    pub fn run(&self) {
        let f = fs::read_to_string(&self.input).expect("error reading file");
        let input = day4::parse_input(f).expect("error parsing input");

        print!(
            "Part 1 Solution: {}\n",
            day4::solve_part_1(input.0, input.1)
        );

        print!(
            "Part 2 Solution: {}\n",
            day4::solve_part_2(input.0, input.1)
        );
    }
}
