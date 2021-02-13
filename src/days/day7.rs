use crate::intcode::{self, IntcodeComputer};
use intcode::ExecutionError;
use permutohedron::heap_recursive;
use rayon::prelude::*;

pub fn solve_part_1(com: &IntcodeComputer) -> Result<(Vec<isize>, isize), ExecutionError> {
    let mut phases: Vec<isize> = (0..5).collect();
    let mut permutations = Vec::new();
    heap_recursive(&mut phases, |p| permutations.push(p.to_vec()));

    let runs = permutations
        .into_par_iter()
        .map(|perm| -> Result<(Vec<isize>, isize), ExecutionError> {
            let mut val = 0;
            perm.iter()
                .try_for_each(|phase| -> Result<(), ExecutionError> {
                    let mut loc_com = com.clone();
                    this_should_not_end_or_output_shit(loc_com.step_pause_on_io())?;
                    this_should_not_end_or_output_shit(loc_com.step(Some(*phase)))?;
                    this_should_not_end_or_output_shit(loc_com.step_pause_on_io())?;
                    this_should_not_end_or_output_shit(loc_com.step(Some(val)))?;
                    val = loc_com.should_output()?;
                    Ok(())
                })?;
            Ok((perm, val))
        })
        .collect::<Result<Vec<(Vec<_>, _)>, _>>()?;

    let max = runs
        .into_par_iter()
        .max_by_key(|run| run.1)
        .ok_or(ExecutionError {
            msg: "could not find max, is none".into(),
            missing_input: false,
        })?;

    Ok(max)
}

pub fn solve_part_2(com: &IntcodeComputer) -> Result<(Vec<isize>, isize), ExecutionError> {
    let mut phases: Vec<isize> = (5..10).collect();
    let mut permutations = Vec::new();
    heap_recursive(&mut phases, |p| permutations.push(p.to_vec()));

    let runs = permutations
        .into_par_iter()
        .map(|perm| -> Result<(Vec<isize>, isize), ExecutionError> {
            let mut val = 0;
            let mut output_val = 0;

            // here we prime the amplifiers
            let mut amplifiers: Vec<_> = perm
                .par_iter()
                .map(|p| -> Result<IntcodeComputer, ExecutionError> {
                    let mut loc_com = com.clone();
                    // find first input node
                    this_should_not_end_or_output_shit(loc_com.step_pause_on_io())?;
                    // give phase setting
                    this_should_not_end_or_output_shit(loc_com.step(Some(*p)))?;
                    // prime it to the next input

                    Ok(loc_com)
                })
                .collect::<Result<_, _>>()?;

            let mut _next = -1;
            let mut get_next = || -> isize {
                _next = _next + 1;
                _next = _next % 5;
                _next
            };

            loop {
                let amp = get_next();
                // should be expecting an input
                if amplifiers[amp as usize].should_stop_on_input()? {
                    return Ok((perm, output_val));
                }
                this_should_not_end_or_output_shit(amplifiers[amp as usize].step(Some(val)))?;
                let next_action = amplifiers[amp as usize].step_pause_on_io()?;
                if next_action.0 {
                    return Ok((perm, output_val));
                }
                val = next_action.1.ok_or_else(|| -> ExecutionError {
                    "not expecting it to ask for input right after".into()
                })?;
                if amp == 4 {
                    output_val = val;
                }
            }
        })
        .collect::<Result<Vec<(Vec<_>, _)>, _>>()?;

    let max = runs
        .into_par_iter()
        .max_by_key(|run| run.1)
        .ok_or(ExecutionError {
            msg: "could not find max, is none".into(),
            missing_input: false,
        })?;

    Ok(max)
}

fn this_should_not_end_or_output_shit(
    r: Result<(bool, Option<isize>), ExecutionError>,
) -> Result<(), ExecutionError> {
    let r = r?;
    if r.0 {
        return Err("this should not end".into());
    };
    if r.1.is_some() {
        return Err("this should not output anything".into());
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::day2::parse_input;
    #[test]
    fn test_part_1() {
        let test_input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string();
        let com = parse_input(test_input).unwrap();
        let res = solve_part_1(&com).unwrap();
        assert_eq!(43210, res.1);
    }

    #[test]
    fn test_part_2() {
        let test_input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .to_string();
        let com = parse_input(test_input).unwrap();
        let res = solve_part_2(&com).unwrap();
        assert_eq!(139629729, res.1);
    }
}
