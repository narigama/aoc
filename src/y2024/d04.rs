use std::collections::HashMap;

use eyre::{Ok, OptionExt};
use itertools::Itertools;

type XY = (usize, usize);

// these offsets operate from the top left corner.
const HORIZONTAL: [XY; 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const VERTICAL: [XY; 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];
const RISING: [XY; 4] = [(0, 3), (1, 2), (2, 1), (3, 0)];
const FALLING: [XY; 4] = [(0, 0), (1, 1), (2, 2), (3, 3)];

pub fn get_input() -> crate::Result<HashMap<XY, char>> {
    Ok(crate::get_input(2024, 4)?
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.char_indices().map(move |(x, ch)| ((x, y), ch)))
        .collect::<HashMap<_, _>>())
}

pub fn get_dimensions(input: &HashMap<XY, char>) -> crate::Result<XY> {
    let x = input.keys().max_by_key(|(x, _)| x).ok_or_eyre("can't find max X")?.0;
    let y = input.keys().max_by_key(|(_, y)| y).ok_or_eyre("can't find max Y")?.1;

    Ok((x, y))
}

pub fn is_xmas(input: &HashMap<XY, char>, x: usize, y: usize, offsets: &[XY]) -> bool {
    let value = offsets.iter().filter_map(|(ox, oy)| input.get(&(x + ox, y + oy))).collect::<String>();
    value == "XMAS" || value == "SAMX"
}

pub fn is_x_mas(input: &HashMap<XY, char>, x: usize, y: usize) -> crate::Result<bool> {
    let upper_left = *input.get(&(x, y)).ok_or_eyre("unable to get x-mas")?;
    let upper_right = *input.get(&(x + 2, y)).ok_or_eyre("unable to get x-mas")?;
    let lower_left = *input.get(&(x, y + 2)).ok_or_eyre("unable to get x-mas")?;
    let lower_right = *input.get(&(x + 2, y + 2)).ok_or_eyre("unable to get x-mas")?;
    let middle = *input.get(&(x + 1, y + 1)).ok_or_eyre("unable to get x-mas")?;

    if middle != 'A' {
        return Ok(false);
    }

    if [upper_left, upper_right, lower_left, lower_right].iter().sorted_unstable().join("") != "MMSS" {
        return Ok(false);
    }

    if upper_left == lower_right {
        return Ok(false);
    }

    Ok(true)
}

pub fn part_one(input: &HashMap<XY, char>) -> crate::Result<u64> {
    let mut count = 0;
    let (width, height) = get_dimensions(input)?;

    for y in 0..=height {
        for x in 0..=width {
            if is_xmas(input, x, y, &HORIZONTAL) {
                count += 1
            }
            if is_xmas(input, x, y, &VERTICAL) {
                count += 1
            }
            if is_xmas(input, x, y, &RISING) {
                count += 1
            }
            if is_xmas(input, x, y, &FALLING) {
                count += 1
            }
        }
    }

    Ok(count)
}

pub fn part_two(input: &HashMap<XY, char>) -> crate::Result<u64> {
    let mut count = 0;
    let (width, height) = get_dimensions(input)?;

    for y in 0..=height {
        for x in 0..=width {
            if is_x_mas(input, x, y).unwrap_or(false) {
                count += 1;
            }
        }
    }

    Ok(count)
}

pub fn main() -> crate::Result<()> {
    let input = get_input()?;

    tracing::info!("y2024d04p01: {}", part_one(&input)?);
    tracing::info!("y2024d04p02: {}", part_two(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y2024d04p01() {
        let input = get_input().unwrap();
        assert_eq!(part_one(&input).unwrap(), 2_583);
    }

    #[test]
    fn y2024d04p02() {
        let input = get_input().unwrap();
        assert_eq!(part_two(&input).unwrap(), 1_978);
    }
}
