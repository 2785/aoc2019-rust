
use std::num::ParseIntError;

pub fn parse_input(f: String) -> Result<Vec<i32>, ParseIntError> {
    f.split("\n")
        .map(|entry| String::from(entry).parse::<i32>())
        .collect()
}

pub fn solve_part1(input: &Vec<i32>) -> i32 {
    input.iter().map(|i| i / 3 - 2).sum::<i32>()
}

pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let mut total = solve_part1(input);
    let mut curr = total;
    while curr > 0 {
        let new = curr / 3 - 2;
        total += new;
        curr = new;
    }

    total
}
