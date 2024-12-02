pub fn get_input() -> crate::Result<Vec<Vec<u64>>> {
    crate::get_input(2024, 2)?
        .trim()
        .lines()
        .map(|line| line.split_ascii_whitespace().map(|number| Ok(number.parse()?)).collect::<crate::Result<Vec<_>>>())
        .collect::<crate::Result<Vec<_>>>()
}

fn check_report(report: &[u64]) -> bool {
    let rising = report.windows(2).all(|w| w[0] < w[1]);
    let falling = report.windows(2).all(|w| w[0] > w[1]);
    let diffs_are_ok = report.windows(2).all(|w| (1..=3).contains(&w[0].abs_diff(w[1])));

    (rising || falling) && diffs_are_ok
}

pub fn part_one(input: &[Vec<u64>]) -> u64 {
    input.iter().filter(|report| check_report(report)).count() as _
}

pub fn part_two(input: &[Vec<u64>]) -> u64 {
    input
        .iter()
        .cloned()
        .filter(|report| {
            // if this report is already safe, skip further checks
            if check_report(&report) {
                return true;
            }

            // walk the reports values, see if it's valid while missing one
            for index in 0..report.len() {
                // make a copy of the report and pop an element.
                let mut report = report.clone();
                report.remove(index);

                // removing a value makes this report work
                if check_report(&report) {
                    return true;
                }
            }

            // no checks passed, fail this report
            false
        })
        .count() as _
}

pub fn main() -> crate::Result<()> {
    let input = get_input()?;

    tracing::info!("y2024d02p01: {}", part_one(&input));
    tracing::info!("y2024d02p02: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y2024d01p01() {
        let input = get_input().unwrap();
        assert_eq!(part_one(&input), 631);
    }

    #[test]
    fn y2024d01p02() {
        let input = get_input().unwrap();
        assert_eq!(part_two(&input), 665);
    }
}
