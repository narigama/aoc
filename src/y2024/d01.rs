use eyre::OptionExt;

use crate::Pair;

pub fn get_input() -> crate::Result<Pair<Vec<u64>>> {
    // make two lists
    let mut left = Vec::new();
    let mut right = Vec::new();

    // for each line, parse the values
    for line in crate::get_input(2024, 1)?.lines() {
        let (l, r) = line.split_once("   ").ok_or_eyre("malformed line: `{line}`")?;
        left.push(l.parse()?);
        right.push(r.parse()?);
    }

    // sort the lists
    left.sort_unstable();
    right.sort_unstable();

    // build a pair and return
    Ok(Pair { left, right })
}

pub fn part_one(input: &Pair<Vec<u64>>) -> u64 {
    input.left.iter().zip(input.right.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
}

pub fn part_two(input: &Pair<Vec<u64>>) -> u64 {
    // count number of occurrences in the left list
    let counts = input.left.iter().fold(std::collections::HashMap::new(), |mut acc, v| {
        *acc.entry(*v).or_insert(0) += 1;
        acc
    });

    // for each number in the right list, if it exists in the left, multiply the two
    input.right.iter().map(|num| if let Some(count) = counts.get(num) { count * (num) } else { 0 }).sum()
}

pub fn main() -> crate::Result<()> {
    let input = get_input()?;

    tracing::info!("y2024d01p01: {}", part_one(&input));
    tracing::info!("y2024d01p02: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y2024d01p01() {
        let input = get_input().unwrap();
        assert_eq!(part_one(&input), 3_714_264);
    }

    #[test]
    fn y2024d01p02() {
        let input = get_input().unwrap();
        assert_eq!(part_two(&input), 18_805_872);
    }
}