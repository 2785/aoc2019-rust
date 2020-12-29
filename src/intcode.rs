use std::collections::HashMap;

#[derive(Clone)]
pub struct IntcodeComputer {
    mem: HashMap<usize, isize>,
    pos: usize,
}

pub fn new(d: Vec<isize>) -> IntcodeComputer {
    let mut map: HashMap<usize, isize> = HashMap::new();
    d.iter().enumerate().for_each(|(i, v)| {
        map.insert(i, *v);
    });

    IntcodeComputer { mem: map, pos: 0 }
}

#[derive(Debug, Clone)]
pub struct PositionNotFoundError {
    pos: usize,
}

#[derive(Debug, Clone)]
pub struct ExecutionError {
    pub msg: String,
}

impl From<PositionNotFoundError> for ExecutionError {
    fn from(e: PositionNotFoundError) -> ExecutionError {
        ExecutionError {
            msg: format!("current pointer at {}, not found in memory", e.pos),
        }
    }
}

impl IntcodeComputer {
    pub fn step(&mut self) -> Result<bool, ExecutionError> {
        let opc = self
            .mem
            .get(&self.pos)
            .ok_or(PositionNotFoundError { pos: self.pos })?;

        match opc {
            1 => {
                let v1 = self.get_val(self.get_pos(1)?)?;
                let v2 = self.get_val(self.get_pos(2)?)?;
                self.set(self.get_pos(3)?, v1 + v2)?;
                self.pos += 4;
                Ok(false)
            }
            2 => {
                let v1 = self.get_val(self.get_pos(1)?)?;
                let v2 = self.get_val(self.get_pos(2)?)?;
                self.set(self.get_pos(3)?, v1 * v2)?;
                self.pos += 4;
                Ok(false)
            }
            99 => Ok(true),
            _ => Err(ExecutionError {
                msg: format!("unknown opcode {}", opc),
            }),
        }
    }

    fn get_pos(&self, shift: usize) -> Result<usize, PositionNotFoundError> {
        let pos = self
            .mem
            .get(&(self.pos + shift))
            .ok_or(PositionNotFoundError {
                pos: self.pos + shift,
            })?;
        Ok(*pos as usize)
    }

    pub fn get_val(&self, pos: usize) -> Result<isize, PositionNotFoundError> {
        let val = self
            .mem
            .get(&pos)
            .ok_or_else(|| PositionNotFoundError { pos: pos })?;
        Ok(*val)
    }

    pub fn set(&mut self, pos: usize, val: isize) -> Result<(), ExecutionError> {
        let res = self.mem.insert(pos, val);
        match res {
            Some(_) => Ok(()),
            None => Err(ExecutionError {
                msg: String::from("could not insert for some reasons"),
            }),
        }
    }
}
