use std::error::Error;

#[derive(Clone, Copy, PartialEq)]
pub enum OpMode {
    Position,
    Immediate,
    Relative,
}

pub fn parse_op_mode(code: isize, count: isize) -> Result<Vec<OpMode>, Box<dyn Error>> {
    (0..count).into_iter().try_fold(vec![], |mut acc, curr| {
        let pos = code % 10isize.pow(curr as u32 + 1) / 10isize.pow(curr as u32);
        let m = match pos {
            0 => OpMode::Position,
            1 => OpMode::Immediate,
            2 => OpMode::Relative,
            _ => {
                return Err(format!("operation mode {} is not supported", pos)
                    .to_string()
                    .into());
            }
        };

        acc.push(m);
        Ok(acc)
    })
}
