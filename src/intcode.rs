use std::{collections::HashMap, error::Error};

use modes::OpMode;

mod modes;

#[derive(PartialEq, Debug)]
pub enum StopEvent {
    Finished,
    WaitingOnInput,
}

#[derive(Clone)]
pub struct IntcodeComputer {
    mem: HashMap<usize, isize>,
    pos: usize,
    relative_base: isize,
}

pub fn new(d: Vec<isize>) -> IntcodeComputer {
    let mut map: HashMap<usize, isize> = HashMap::new();
    d.iter().enumerate().for_each(|(i, v)| {
        map.insert(i, *v);
    });

    IntcodeComputer {
        mem: map,
        pos: 0,
        relative_base: 0,
    }
}

#[derive(Debug, Clone)]
pub struct PositionNotFoundError {
    pos: usize,
}

#[derive(Debug, Clone)]
pub struct ExecutionError {
    pub msg: String,
    pub missing_input: bool,
}

impl From<PositionNotFoundError> for ExecutionError {
    fn from(e: PositionNotFoundError) -> ExecutionError {
        ExecutionError {
            msg: format!("current pointer at {}, not found in memory", e.pos),
            missing_input: false,
        }
    }
}

impl From<Box<dyn Error>> for ExecutionError {
    fn from(e: Box<dyn Error>) -> ExecutionError {
        ExecutionError {
            msg: e.to_string(),
            missing_input: false,
        }
    }
}

impl From<String> for ExecutionError {
    fn from(e: String) -> ExecutionError {
        ExecutionError {
            msg: e,
            missing_input: false,
        }
    }
}

impl From<&str> for ExecutionError {
    fn from(s: &str) -> ExecutionError {
        ExecutionError {
            msg: s.to_string(),
            missing_input: false,
        }
    }
}

impl IntcodeComputer {
    pub fn step(&mut self, input: Option<isize>) -> Result<(bool, Option<isize>), ExecutionError> {
        let opc = self
            .mem
            .get(&self.pos)
            .ok_or(PositionNotFoundError { pos: self.pos })?;

        match opc % 100 {
            1 => {
                let mode = modes::parse_op_mode(opc / 100, 3)?;
                let v1 = self.get_param(1, mode[0])?;
                let v2 = self.get_param(2, mode[1])?;
                let v3 = self.get_param_write(3, mode[2])?;
                self.set(v3, v1 + v2);
                self.pos += 4;
                Ok((false, None))
            }
            2 => {
                let mode = modes::parse_op_mode(opc / 100, 3)?;
                let v1 = self.get_param(1, mode[0])?;
                let v2 = self.get_param(2, mode[1])?;
                let v3 = self.get_param_write(3, mode[2])?;
                self.set(v3, v1 * v2);
                self.pos += 4;
                Ok((false, None))
            }
            3 => {
                let mode = modes::parse_op_mode(opc / 100, 1)?;
                let v1 = self.get_param_write(1, mode[0])?;
                let input = input.ok_or_else(|| ExecutionError {
                    msg: "missing input".to_string(),
                    missing_input: true,
                })?;
                self.set(v1, input);
                self.pos += 2;
                Ok((false, None))
            }
            4 => {
                let mode = modes::parse_op_mode(opc / 100, 1)?;
                let v1 = self.get_param(1, mode[0])?;
                self.pos += 2;
                Ok((false, Some(v1)))
            }
            5 => {
                let mode = modes::parse_op_mode(opc / 100, 2)?;
                let v1 = self.get_param(1, mode[0])?;
                let v2 = self.get_param(2, mode[1])?;
                if v1 != 0 {
                    self.pos = v2 as usize;
                } else {
                    self.pos += 3;
                }
                Ok((false, None))
            }
            6 => {
                let mode = modes::parse_op_mode(opc / 100, 2)?;
                let v1 = self.get_param(1, mode[0])?;
                let v2 = self.get_param(2, mode[1])?;
                if v1 == 0 {
                    self.pos = v2 as usize;
                } else {
                    self.pos += 3;
                }
                Ok((false, None))
            }
            7 => {
                let mode = modes::parse_op_mode(opc / 100, 3)?;
                let v1 = self.get_param(1, mode[0])?;
                let v2 = self.get_param(2, mode[1])?;
                let v3 = self.get_param_write(3, mode[2])?;
                self.set(v3, if v1 < v2 { 1 } else { 0 });
                self.pos += 4;
                Ok((false, None))
            }
            8 => {
                let mode = modes::parse_op_mode(opc / 100, 3)?;
                let v1 = self.get_param(1, mode[0])?;
                let v2 = self.get_param(2, mode[1])?;
                let v3 = self.get_param_write(3, mode[2])?;
                self.set(v3, if v1 == v2 { 1 } else { 0 });
                self.pos += 4;
                Ok((false, None))
            }
            9 => {
                let mode = modes::parse_op_mode(opc / 100, 1)?;
                let v1 = self.get_param(1, mode[0])?;
                self.relative_base += v1;
                self.pos += 2;
                Ok((false, None))
            }
            99 => Ok((true, None)),
            _ => Err(ExecutionError {
                msg: format!("unknown opcode {}", opc),
                missing_input: false,
            }),
        }
    }

    pub fn step_pause_on_io(&mut self) -> Result<(bool, Option<isize>), ExecutionError> {
        loop {
            match self.step(None) {
                Ok(opt) => {
                    if opt.0 == true {
                        return Ok((true, None));
                    }
                    if opt.1.is_some() {
                        return Ok((false, opt.1));
                    }
                }
                Err(e) => {
                    if e.missing_input {
                        return Ok((false, None));
                    }
                }
            }
        }
    }

    pub fn should_output(&mut self) -> Result<isize, ExecutionError> {
        let r = self.step_pause_on_io()?;
        if (r).0 {
            return Err("this should not end".into());
        };
        (r.1).ok_or("expecting value".into())
    }

    pub fn should_stop_on_input(&mut self) -> Result<bool, ExecutionError> {
        let r = self.step_pause_on_io()?;
        if (r.1).is_some() {
            Err("not expecting output".into())
        } else {
            Ok(r.0)
        }
    }

    pub fn accumulate_output_until_action(
        &mut self,
    ) -> Result<(Vec<isize>, StopEvent), ExecutionError> {
        let mut outputs = Vec::new();
        loop {
            let r = self.step_pause_on_io()?;
            if r.0 {
                return Ok((outputs, StopEvent::Finished));
            }
            match r.1 {
                Some(v) => outputs.push(v),
                None => {
                    return Ok((outputs, StopEvent::WaitingOnInput));
                }
            }
        }
    }

    fn get_param(&mut self, shift: usize, mode: OpMode) -> Result<isize, PositionNotFoundError> {
        match mode {
            OpMode::Position => {
                let pos = self.get_pos(shift)?;
                self.get_val(pos)
            }
            OpMode::Immediate => self.get_val(self.pos + shift),
            OpMode::Relative => {
                let pos = (self.get_pos(shift)? as isize + self.relative_base) as usize;
                self.get_val(pos)
            }
        }
    }

    fn get_param_write(&mut self, shift: usize, mode: OpMode) -> Result<usize, ExecutionError> {
        let loc = self.get_pos(shift)?;
        match mode {
            OpMode::Immediate => Err("input parameter cannot be in immediate mode".into()),
            OpMode::Position => Ok(loc),
            OpMode::Relative => Ok((loc as isize + self.relative_base) as usize),
        }
    }

    fn get_pos(&mut self, shift: usize) -> Result<usize, PositionNotFoundError> {
        let val = self.mem.get(&(self.pos + shift));

        match val {
            Some(v) => Ok(*v as usize),
            None => {
                self.mem.insert(self.pos + shift, 0);
                Ok(0)
            }
        }
    }

    pub fn get_val(&mut self, pos: usize) -> Result<isize, PositionNotFoundError> {
        let val = self.mem.get(&pos);

        match val {
            Some(v) => Ok(*v),
            None => {
                self.mem.insert(pos, 0);
                Ok(0)
            }
        }
    }

    pub fn set(&mut self, pos: usize, val: isize) {
        self.mem.insert(pos, val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_io() {
        let mut com = new(vec![3, 0, 4, 0, 99]);

        loop {
            let out = com.step(Some(5)).expect("Error executing machine");

            if out.1.is_some() {
                // print!("{}", out.1.unwrap());
                assert_eq!(5, out.1.unwrap());
            }

            if out.0 {
                break;
            }
        }
    }

    #[test]
    fn test_day9() {
        // let mut com = new(vec![104, 1125899906842624, 99]);
        // assert_eq!(com.should_output().unwrap(), 1125899906842624);

        let mut com = new(vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]);
        let (out, event) = com.accumulate_output_until_action().unwrap();
        assert_eq!(event, StopEvent::Finished);
        assert_eq!(
            out,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        )
    }
}
