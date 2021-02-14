use ndarray::parallel::prelude::*;
use ndarray::prelude::*;
use ndarray::{Array, Axis};
use std::error::Error;

pub fn parse_input(
    f: String,
    width: usize,
    height: usize,
) -> Result<Array<u32, Ix3>, Box<dyn Error>> {
    let digits = f
        .chars()
        .collect::<Vec<_>>()
        .par_iter()
        .map(|b| -> Result<_, _> {
            b.to_digit(10)
                .ok_or_else(|| format!("cannot parse {} into integer", b))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if digits.len() % (height * width) != 0 {
        return Err(format!(
            "expecting width {}, height {}, total {} elements, got {}",
            height,
            width,
            height * width,
            digits.len()
        )
        .into());
    };

    let arr = Array::from_shape_vec((digits.len() / (height * width), height, width), digits)?;

    Ok(arr)
}

pub fn solve_part_1(arr: &Array<u32, Ix3>) -> Result<usize, Box<dyn Error>> {
    let minimum_zeros = arr
        .axis_iter(Axis(0))
        .into_par_iter()
        .min_by_key(|layer| layer.iter().filter(|t| **t == 0).count())
        .ok_or_else(|| "could not filter".to_string())?;

    let ones = minimum_zeros.iter().filter(|t| **t == 1).count();
    let twos = minimum_zeros.iter().filter(|t| **t == 2).count();
    Ok(ones * twos)
}

pub fn solve_part_2(arr: &Array<u32, Ix3>) -> Result<String, Box<dyn Error>> {
    let mut img = Array::from_elem((arr.shape()[1], arr.shape()[2]), 0 as u32);
    let h = img.shape()[0];
    let w = img.shape()[1];
    for i in 0..h {
        for j in 0..w {
            let col = arr.slice(s![.., i, j]);
            match col.iter().find(|x| **x != 2) {
                Some(v) => img[[i, j]] = *v,
                None => img[[i, j]] = 2,
            };
        }
    }
    let joined = img
        .axis_iter(Axis(0))
        .map(|row| {
            row.into_iter()
                .map(|e| match e {
                    0 => ".",
                    1 => "#",
                    2 => " ",
                    _ => panic!("can't happen"),
                })
                .collect::<Vec<&str>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
        .to_string();

    Ok(joined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "123456345678".to_string();
        assert_eq!(
            parse_input(input, 3, 2).unwrap(),
            array![[[1, 2, 3], [4, 5, 6],], [[3, 4, 5], [6, 7, 8]],]
        )
    }
}
