use eyre::OptionExt;
use rayon::prelude::*;

use crate::util::ProductRepeat;

pub enum Operation {
    Add,
    Mul,
    Concat,
}

pub type Equation = (u64, Vec<u64>);

pub fn get_input() -> crate::Result<Vec<Equation>> {
    crate::get_input(2024, 7)?
        .trim()
        .lines()
        .map(|line| {
            let (target, values) = line.split_once(": ").ok_or_eyre("Can't split equation")?;
            let target = target.parse()?;
            let values = values.split(' ').map(|value| Ok(value.parse()?)).collect::<crate::Result<Vec<_>>>()?;

            Ok((target, values))
        })
        .collect::<crate::Result<Vec<_>>>()
}

pub fn can_be_solved(target: u64, values: &[u64], operations: &[Operation]) -> bool {
    for permutation in operations.iter().product_repeat(values.len() - 1) {
        let mut operators = permutation.iter();

        let result = values
            .iter()
            .copied()
            // SAFE: product_repeat guarantees enough operators based on the number of values
            .reduce(|a, b| match operators.next().unwrap() {
                Operation::Add => a + b,
                Operation::Mul => a * b,
                // SAFE: we know the inputs are unsigned ints and they'll always concat correctly
                Operation::Concat => format!("{a}{b}").parse().unwrap(),
            })
            .unwrap();

        if result == target {
            return true;
        }
    }

    false
}

pub fn part_one(input: &[Equation]) -> u64 {
    input
        .par_iter()
        .filter_map(|(target, values)| {
            can_be_solved(*target, values, &[Operation::Add, Operation::Mul]).then_some(target)
        })
        .sum()
}

pub fn part_two(input: &[Equation]) -> u64 {
    input
        .par_iter()
        .filter_map(|(target, values)| {
            can_be_solved(*target, values, &[Operation::Add, Operation::Mul, Operation::Concat]).then_some(target)
        })
        .sum()
}

pub fn main() -> crate::Result<()> {
    let input = get_input()?;

    tracing::info!("y2024d07p01: {}", part_one(&input));
    tracing::info!("y2024d07p02: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y2024d07p01() {
        let input = get_input().unwrap();
        assert_eq!(part_one(&input), 3_245_122_495_150);
    }

    #[test]
    fn y2024d07p02() {
        let input = get_input().unwrap();
        assert_eq!(part_two(&input), 105_517_128_211_543);
    }
}
