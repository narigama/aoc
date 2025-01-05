use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point(pub i64, pub i64);

impl Point {
    pub fn within_bounds(&self, width: i64, height: i64) -> bool {
        0 <= self.0 && self.0 <= width && 0 <= self.1 && self.1 <= height
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub map: HashMap<char, HashSet<Point>>,
    pub width: i64,
    pub height: i64,
}

pub fn get_input() -> crate::Result<Input> {
    let input = crate::get_input(2024, 8)?;

    let mut map: HashMap<char, HashSet<Point>> = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.trim().lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                map.entry(char).or_default().insert(Point(x as _, y as _));
            }

            width = width.max(x);
        }
        height = height.max(y);
    }

    eyre::ensure!(width == height, "grid isn't square!");

    Ok(Input { map, width: width as _, height: height as _ })
}

pub fn part_one(input: &Input) -> u64 {
    let mut antinodes = HashSet::<Point>::new();

    for points in input.map.values() {
        for (a, b) in points.iter().cloned().tuple_combinations() {
            let diff = a - b;

            for point in [a - diff, a + diff, b - diff, b + diff] {
                if point.within_bounds(input.width, input.height) && point != a && point != b {
                    antinodes.insert(point);
                }
            }
        }
    }

    antinodes.len() as _
}

pub fn part_two(input: &Input) -> u64 {
    let mut antinodes = HashSet::new();

    for points in input.map.values() {
        for (a, b) in points.iter().cloned().tuple_combinations() {
            let diff = a - b;

            // walk forwards from a until out of bounds
            let mut c = a;
            while c.within_bounds(input.width, input.height) {
                antinodes.insert(c);
                c = c + diff;
            }

            // walk backwards from a until out of bounds
            let mut c = a;
            while c.within_bounds(input.width, input.height) {
                antinodes.insert(c);
                c = c - diff;
            }

            // walk forwards from b until out of bounds
            let mut c = b;
            while c.within_bounds(input.width, input.height) {
                antinodes.insert(c);
                c = c + diff;
            }

            // walk backwards from b until out of bounds
            let mut c = b;
            while c.within_bounds(input.width, input.height) {
                antinodes.insert(c);
                c = c - diff;
            }
        }
    }

    antinodes.len() as _
}

pub fn main() -> crate::Result<()> {
    let input = get_input()?;

    tracing::info!("y2024d08p01: {}", part_one(&input));
    tracing::info!("y2024d08p02: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y2024d08p01() {
        let input = get_input().unwrap();
        assert_eq!(part_one(&input), 351);
    }

    #[test]
    fn y2024d08p02() {
        let input = get_input().unwrap();
        assert_eq!(part_two(&input), 1_259);
    }
}
