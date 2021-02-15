use intcode::ExecutionError;
use rayon::prelude::*;
use std::num::ParseIntError;

use crate::intcode::{self, IntcodeComputer};

pub fn parse_input(f: String) -> Result<IntcodeComputer, ParseIntError> {
    let data = f
        .split(",")
        .collect::<Vec<&str>>()
        .par_iter()
        .map(|s| String::from(*s).parse::<isize>().expect("cannot parse int"))
        .collect();
    Ok(intcode::new(data))
}

pub fn solve_part_1(com: &mut IntcodeComputer) -> Result<isize, intcode::ExecutionError> {
    com.set(1, 12);
    com.set(2, 2);

    while !com.step(None)?.0 {}

    Ok(com.get_val(0)?)
}

pub fn solve_part_2(com: &mut IntcodeComputer) -> Result<isize, intcode::ExecutionError> {
    let mut inputs: Vec<(isize, isize)> = vec![];
    for noun in 0..100 {
        for verb in 0..100 {
            inputs.push((noun, verb));
        }
    }

    let found = inputs
        .par_iter()
        .find_any(|(n, v)| {
            let mut c = com.clone();
            c.set(1, *n);
            c.set(2, *v);
            while !c.step(None).expect("error stepping in intcode com").0 {}
            c.get_val(0)
                .expect("error getting pos 0, this should not happen")
                == 19690720
        })
        .ok_or_else(|| ExecutionError {
            msg: format!("requested value not found"),
            missing_input: false,
        })?;

    Ok(found.0 * 10 + found.1)
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &'static str = "1,9,10,3,2,3,11,0,99,30,40,50";
    use super::parse_input;

    #[test]
    fn parses_fine() {
        let res = parse_input(String::from(TEST_INPUT));
        assert!(res.is_ok());
    }
}
