use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

pub struct Node {
    pub id: String,
    pub dir: String,
    pub dist: Option<isize>,
}

pub fn parse_input(f: String) -> Result<HashMap<String, Node>, Box<dyn Error>> {
    let mut mapper = f
        .par_split('\n')
        .map(|line| {
            let parts = line
                .split(')')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            if parts.len() != 2 {
                Err(format!(
                    "unexpected number of segments in line {}, expecting 2, got {}",
                    line,
                    parts.len()
                ))
            } else {
                Ok((
                    parts[1].clone(),
                    Node {
                        id: parts[1].clone(),
                        dir: parts[0].clone(),
                        dist: None,
                    },
                ))
            }
        })
        .collect::<Result<HashMap<String, Node>, _>>()?;

    let keys = mapper.keys().cloned().collect::<Vec<_>>();

    for k in keys {
        evaluate(&k, &mut mapper)?;
    }

    Ok(mapper)
}

fn evaluate(n: &String, l: &mut HashMap<String, Node>) -> Result<isize, Box<dyn Error>> {
    let curr = l
        .get(n)
        .ok_or_else::<String, _>(|| format!("{} not found in mapper", n).into())?;

    if curr.dist.is_some() {
        return Ok(curr.dist.unwrap());
    }

    let dir = curr.dir.clone();

    if dir == "COM" {
        l.get_mut(n).unwrap().dist = Some(1);
        return Ok(1);
    }

    let parent = evaluate(&dir, l)?;

    l.get_mut(n).unwrap().dist = Some(parent + 1);

    Ok(parent + 1)
}

pub fn solve_part_1(mapper: &HashMap<String, Node>) -> isize {
    mapper
        .iter()
        .fold(0, |acc, curr| acc + curr.1.dist.unwrap())
}

pub fn solve_part_2(mapper: &HashMap<String, Node>) -> Result<isize, Box<dyn Error>> {
    let mut you_set: HashSet<String> = HashSet::new();
    let mut curr = "YOU".to_string();
    loop {
        let next = mapper
            .get(&mapper.get(&curr).ok_or("curr not found")?.dir)
            .ok_or("next not found")?;
        if next.dir != "COM".to_string() {
            you_set.insert(next.id.clone());
            curr = next.id.clone();
        } else {
            break;
        }
    }

    let mut san_set = HashSet::new();
    let mut curr = "SAN".to_string();
    loop {
        let next = mapper
            .get(&mapper.get(&curr).ok_or("curr not found")?.dir)
            .ok_or("next not found")?;
        if next.dir != "COM".to_string() {
            san_set.insert(next.id.clone());
            curr = next.id.clone();
        } else {
            break;
        }
    }
    let from_root = you_set.intersection(&san_set).count();

    Ok((you_set.len() - from_root + san_set.len() - from_root) as isize)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_io() {
        let test_input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let parsed = parse_input(test_input.to_string()).expect("cannot parse input");
        assert_eq!(42, solve_part_1(&parsed));
    }

    #[test]
    fn test_part_2() {
        let test_input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let parsed = parse_input(test_input.to_string()).expect("cannot parse input");
        assert_eq!(4, solve_part_2(&parsed).unwrap());
    }
}
