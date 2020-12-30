use rayon::prelude::*;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up(isize),
    Down(isize),
    Left(isize),
    Right(isize),
}

pub fn parse_input(f: String) -> Result<(Vec<Direction>, Vec<Direction>), Box<dyn Error>> {
    let split = f.split("\n").collect::<Vec<&str>>();
    if split.len() != 2 {
        return Err(format!(
            "unexpected number of lines in file, expecting 2, got {}",
            split.len()
        )
        .into());
    }

    let re = Regex::new(r"(?P<dir>\w)(?P<len>\d+)")?;

    let mut parsed = split
        .par_iter()
        .map(|l| {
            l.par_split(',')
                .map(|x| -> Result<Direction, String> {
                    let cap = re.captures(x).ok_or("cannot capture")?;
                    let num = cap
                        .name("len")
                        .ok_or("len not found")?
                        .as_str()
                        .parse::<isize>();

                    if num.is_err() {
                        return Err(format!(
                            "could not parse string into int: {}",
                            num.unwrap_err()
                        ));
                    }

                    let num = num.unwrap();

                    match cap.name("dir").ok_or("dir not found")?.as_str() {
                        "U" => Ok(Direction::Up(num)),
                        "D" => Ok(Direction::Down(num)),
                        "L" => Ok(Direction::Left(num)),
                        "R" => Ok(Direction::Right(num)),
                        _ => Err(format!(
                            "unrecognizable direction: {}",
                            cap.name("dir").unwrap().as_str()
                        )),
                    }
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?;

    let o2 = parsed.pop().unwrap();
    let o1 = parsed.pop().unwrap();

    Ok((o1, o2))
}

pub fn solve_part_1(one: &Vec<Direction>, two: &Vec<Direction>) -> isize {
    use Direction::*;

    let collect_set = |d: &Vec<Direction>| -> HashSet<(isize, isize)> {
        let mut s = HashSet::new();
        d.clone()
            .iter()
            .fold::<(isize, isize), _>((0, 0), |mut acc, curr| {
                let (val, diff) = match curr {
                    Up(x) => (x, (0, 1)),
                    Down(x) => (x, (0, -1)),
                    Left(x) => (x, (-1, 0)),
                    Right(x) => (x, (1, 0)),
                };

                for _ in 0..*val {
                    acc.0 += diff.0;
                    acc.1 += diff.1;
                    s.insert(acc);
                }
                acc
            });
        s
    };

    let s1 = collect_set(one);
    let s2 = collect_set(two);

    let intersect = s1.intersection(&s2);

    let min_distance = intersect
        .min_by_key(|p| (if p.0 < 0 { -p.0 } else { p.0 }) + (if p.1 < 1 { -p.1 } else { p.1 }))
        .expect("intersection is empty");

    (if min_distance.0 < 0 {
        -min_distance.0
    } else {
        min_distance.0
    }) + (if min_distance.1 < 1 {
        -min_distance.1
    } else {
        min_distance.1
    })
}

pub fn solve_part_2(one: &Vec<Direction>, two: &Vec<Direction>) -> isize {
    use Direction::*;

    let collect_set = |d: &Vec<Direction>| -> HashMap<(isize, isize), isize> {
        let mut s = HashMap::new();
        d.clone()
            .iter()
            .fold::<(isize, isize, isize), _>((0, 0, 0), |mut acc, curr| {
                let (val, diff) = match curr {
                    Up(x) => (x, (0, 1)),
                    Down(x) => (x, (0, -1)),
                    Left(x) => (x, (-1, 0)),
                    Right(x) => (x, (1, 0)),
                };

                for _ in 0..*val {
                    acc.0 += diff.0;
                    acc.1 += diff.1;
                    acc.2 += 1;
                    if !s.contains_key(&(acc.0, acc.1)) {
                        s.insert((acc.0, acc.1), acc.2);
                    }
                }
                acc
            });
        s
    };

    let m1 = collect_set(one);
    let m2 = collect_set(two);

    let s1: HashSet<(isize, isize)> = m1.keys().cloned().collect();
    let s2: HashSet<(isize, isize)> = m2.keys().cloned().collect();

    let intersect = s1.intersection(&s2);

    let min_distance = intersect
        .min_by_key(|p| (m1.get(p).unwrap() + m2.get(p).unwrap()))
        .expect("intersection is empty");

    m1.get(min_distance).unwrap() + m2.get(min_distance).unwrap()
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &'static str =
        "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    use super::Direction::*;
    use super::{parse_input, solve_part_1, solve_part_2};

    #[test]
    fn parses_fine() {
        let res = parse_input(String::from(TEST_INPUT));
        assert!(res.is_ok());
    }

    #[test]
    fn test_direction() {
        assert_eq!(Right(10), Right(10));
        assert_ne!(Right(5), Right(10));
    }

    #[test]
    fn test_solve_part_1() {
        let res = parse_input(TEST_INPUT.to_string()).unwrap();
        assert_eq!(159, solve_part_1(&res.0, &res.1));
    }

    #[test]
    fn test_solve_part_2() {
        let res = parse_input(TEST_INPUT.to_string()).unwrap();
        assert_eq!(610, solve_part_2(&res.0, &res.1));
    }
}
