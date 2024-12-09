use std::collections::{HashMap, HashSet};

use eyre::OptionExt;

type Rules = HashMap<u64, HashSet<u64>>;

pub struct Input {
    pub rules: Rules,
    pub updates: Vec<Vec<u64>>,
}

fn is_correct(update: &[u64], rules: &Rules) -> bool {
    for (key, values) in rules {
        for value in values {
            if update.contains(key) && update.contains(value) {
                let idx_left = update.iter().position(|v| v == key).unwrap_or(usize::MAX);
                let idx_right = update.iter().position(|v| v == value).unwrap_or(usize::MIN);

                if idx_left >= idx_right {
                    return false;
                }
            }
        }
    }

    true
}

pub fn get_input() -> crate::Result<Input> {
    let input = crate::get_input(2024, 5)?;
    let (rules_raw, updates_raw) = input.split_once("\n\n").ok_or_eyre(r"Unable to split input at \n\n")?;

    let mut rules: Rules = HashMap::new();

    for rule in rules_raw.trim().lines() {
        let (left, right) = rule.split_once('|').ok_or_eyre("unable to split rule on |")?;
        let (left, right) = (left.parse()?, right.parse()?);
        rules.entry(left).or_default().insert(right);
    }

    let updates = updates_raw
        .trim()
        .lines()
        .map(|line| line.split(',').map(|value| Ok(value.parse()?)).collect::<crate::Result<Vec<_>>>())
        .collect::<crate::Result<Vec<Vec<_>>>>()?;

    Ok(Input { rules, updates })
}

pub fn part_one(input: &Input) -> u64 {
    let mut count = 0;

    for update in &input.updates {
        if is_correct(update, &input.rules) {
            count += update[update.len() / 2];
        }
    }

    count
}

pub fn part_two(input: &Input) -> u64 {
    let mut count = 0;

    for update in &input.updates {
        if !is_correct(update, &input.rules) {
            // repeatedly apply the rules until this rule is correct
            let mut update = update.clone();

            loop {
                let mut changed = false;
                for (key, values) in &input.rules {
                    for value in values {
                        if update.contains(key) && update.contains(value) {
                            let idx_left = update.iter().position(|v| v == key).unwrap_or(usize::MAX);
                            let idx_right = update.iter().position(|v| v == value).unwrap_or(usize::MIN);

                            if idx_left >= idx_right {
                                update.swap(idx_left, idx_right);
                                changed = true
                            }
                        }
                    }
                }

                // every rule passed, this update is now correct
                if !changed {
                    break;
                }
            }
            count += update[update.len() / 2];
        }
    }

    count
}

pub fn main() -> crate::Result<()> {
    let input = get_input()?;

    tracing::info!("y2024d05p01: {}", part_one(&input));
    tracing::info!("y2024d05p02: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y2024d05p01() {
        let input = get_input().unwrap();
        assert_eq!(part_one(&input), 6_242);
    }

    #[test]
    fn y2024d05p02() {
        let input = get_input().unwrap();
        assert_eq!(part_two(&input), 5_169);
    }
}
