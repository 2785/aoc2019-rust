use rayon::prelude::*;
use std::error::Error;

pub fn parse_input(f: String) -> Result<(isize, isize), Box<dyn Error>> {
    let split: Vec<String> = f.split("-").into_iter().map(|x| x.to_string()).collect();
    if split.len() != 2 {
        return Err(format!(
            "unexpected number of segments, expected 2, got {}",
            split.len()
        )
        .into());
    };

    let ints: Vec<isize> = split
        .iter()
        .map(|x| x.parse::<isize>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok((ints[0].clone(), ints[1].clone()))
}

pub fn solve_part_1(lo: isize, hi: isize) -> Result<isize, Box<dyn Error>> {
    let mut counter = 0;

    for n in lo..hi + 1 {
        // break down to parts
        let digits: Vec<u8> = n
            .to_string()
            .split("")
            .into_iter()
            .filter_map(|x| x.parse::<u8>().ok())
            .collect::<Vec<u8>>();

        let this_is_fine = digits
            .iter()
            .try_fold((0u8, 1u8, false), |acc, curr| -> Option<_> {
                if *curr < acc.0 {
                    return None;
                };

                if !acc.2 {
                    if *curr != acc.0 && acc.1 >= 2 {
                        return Some((*curr, 0, true));
                    };

                    if *curr == acc.0 {
                        return Some((*curr, acc.1 + 1, false));
                    };
                };

                Some((*curr, 1, acc.2))
            });

        if this_is_fine.is_some() {
            let this_is_fine = this_is_fine.unwrap();
            if this_is_fine.2 || this_is_fine.1 >= 2 {
                counter += 1;
            };
        };
    }

    Ok(counter)
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &'static str = "172851-675869";
    use super::*;

    #[test]
    fn parses_fine() {
        let res = parse_input(String::from(TEST_INPUT));
        assert!(res.is_ok());
        assert_eq!((172851, 675869), res.unwrap());
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(1660, solve_part_1(172851, 675869).expect("got err"));
    }

    // #[test]
    // fn test_solve_part_2() {
    //     let res = parse_input(TEST_INPUT.to_string()).unwrap();
    //     assert_eq!(610, solve_part_2(&res.0, &res.1));
    // }
}
