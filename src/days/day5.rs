use crate::intcode::{self, IntcodeComputer};

pub fn solve_part_1(com: &mut IntcodeComputer) -> Result<isize, intcode::ExecutionError> {
    com.step(Some(1))?;
    loop {
        let res = com.step(None)?;
        if res.1.is_some() {
            let val = res.1.unwrap();
            if val != 0 {
                return Ok(val);
            }
        }
        if res.0 {
            break;
        }
    }

    Ok(0)
}

pub fn solve_part_2(com: &mut IntcodeComputer) -> Result<isize, intcode::ExecutionError> {
    com.step(Some(5))?;
    loop {
        let res = com.step(None)?;
        if res.1.is_some() {
            let val = res.1.unwrap();
            if val != 0 {
                return Ok(val);
            }
        }
        if res.0 {
            break;
        }
    }

    Ok(0)
}
