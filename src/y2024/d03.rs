use eyre::OptionExt;

pub enum Instruction {
    Enable,
    Disable,
    Mul(u64, u64),
}

pub fn get_input() -> crate::Result<Vec<Instruction>> {
    let input = crate::get_input(2024, 3)?;

    // already on the regex
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")?;
    re.captures_iter(&input)
        .map(|match_| {
            let capture = match_.get(0).ok_or_eyre("bad match")?.as_str();
            let (op, _) = capture.split_once("(").ok_or_eyre("bad op")?;

            match op {
                "do" => Ok(Instruction::Enable),
                "don't" => Ok(Instruction::Disable),
                other => {
                    if other.starts_with("mul") {
                        let a = match_.get(1).ok_or_eyre("bad match")?.as_str().parse()?;
                        let b = match_.get(2).ok_or_eyre("bad match")?.as_str().parse()?;
                        Ok(Instruction::Mul(a, b))
                    } else {
                        Err(eyre::eyre!("unknown operator: {other}"))
                    }
                }
            }
        })
        .collect::<crate::Result<Vec<_>>>()
}

pub fn part_one(input: &[Instruction]) -> u64 {
    input.iter().map(|op| if let Instruction::Mul(a, b) = op { a * b } else { 0 }).sum()
}

pub fn part_two(input: &[Instruction]) -> u64 {
    let mut count = 0;
    let mut enabled = true;

    for op in input {
        match op {
            Instruction::Enable => enabled = true,
            Instruction::Disable => enabled = false,
            Instruction::Mul(a, b) => {
                if enabled {
                    count += a * b;
                }
            }
        }
    }
    count
}

pub fn main() -> crate::Result<()> {
    let input = get_input()?;

    tracing::info!("y2024d03p01: {}", part_one(&input));
    tracing::info!("y2024d03p02: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y2024d03p01() {
        let input = get_input().unwrap();
        assert_eq!(part_one(&input), 153_469_856);
    }

    #[test]
    fn y2024d03p02() {
        let input = get_input().unwrap();
        assert_eq!(part_two(&input), 77_055_967);
    }
}
