use crate::intcode::{ExecutionError, IntcodeComputer, StopEvent};

pub fn solve_part_1(com: &IntcodeComputer) -> Result<isize, ExecutionError> {
    let mut loc_com = com.clone();
    if loc_com.should_stop_on_input()? {
        return Err("expecting input".into());
    };

    loc_com.step(Some(1))?;

    let (outputs, stop_mode) = loc_com.accumulate_output_until_action()?;

    if stop_mode != StopEvent::Finished {
        return Err("expecting to output values until finishing".into());
    };

    if outputs.len() != 1 {
        println!(
            "outputs:\n {}",
            outputs
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<_>>()
                .join("\n")
        );
        return Err(format!("expecting one single output, got {}", outputs.len()).into());
    };

    Ok(outputs[0])
}

pub fn solve_part_2(com: &IntcodeComputer) -> Result<isize, ExecutionError> {
    let mut loc_com = com.clone();
    if loc_com.should_stop_on_input()? {
        return Err("expecting input".into());
    };

    loc_com.step(Some(2))?;

    let (outputs, stop_mode) = loc_com.accumulate_output_until_action()?;

    if stop_mode != StopEvent::Finished {
        return Err("expecting to output values until finishing".into());
    };

    if outputs.len() != 1 {
        println!(
            "outputs:\n {}",
            outputs
                .iter()
                .map(|t| format!("{}", t))
                .collect::<Vec<_>>()
                .join("\n")
        );
        return Err(format!("expecting one single output, got {}", outputs.len()).into());
    };

    Ok(outputs[0])
}
