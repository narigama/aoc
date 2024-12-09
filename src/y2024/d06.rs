use std::collections::{HashMap, HashSet};

pub type Position = (i64, i64);

#[derive(Debug, Clone)]
pub enum Tile {
    Floor,
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            _ => Self::Floor,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum GuardDirection {
    #[default]
    North,
    South,
    East,
    West,
}

impl GuardDirection {
    pub fn get_offset(&self) -> Position {
        match self {
            GuardDirection::North => (0, -1),
            GuardDirection::South => (0, 1),
            GuardDirection::East => (1, 0),
            GuardDirection::West => (-1, 0),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Guard {
    position: Position,
    direction: GuardDirection,
}

#[derive(Debug, Clone)]
pub struct Input {
    map: HashMap<Position, Tile>,
    guard: Guard,
}

pub enum StopCondition {
    Exited(u64), // tiles walked
    Looped,      // ended up in an infinite loop
}

pub fn get_input() -> crate::Result<Input> {
    let mut map = HashMap::new();
    let mut guard = Guard::default();

    for (y, line) in crate::get_input(2024, 6)?.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as _, y as _), c.into());

            if c == '^' {
                // move the guard to this starting position
                guard.position = (x as _, y as _);
            }
        }
    }

    Ok(Input { map, guard })
}

fn run_simulation(map: HashMap<Position, Tile>, mut guard: Guard) -> StopCondition {
    // mark tiles as we visit, start with the guard's initial position
    let mut visited = HashSet::new();

    loop {
        visited.insert(guard.position);

        // look at the next tile
        let offset = guard.direction.get_offset();
        let position = (guard.position.0 + offset.0, guard.position.1 + offset.1);

        // have we visited that tile?
        // if visited.contains(&position) {
        //     tracing::info!("Guard looks like it's looping. Stopping...");
        //     return StopCondition::Looped;
        // }

        match map.get(&position) {
            None => {
                tracing::info!("Found empty space ({:?}). Stopping.", position);
                return StopCondition::Exited(visited.len() as _);
            }

            Some(tile) => match tile {
                Tile::Floor => {
                    // walk forwards
                    tracing::info!("Guard moved to {:?}", position);
                    guard.position = position;
                }
                Tile::Wall => {
                    // turn 90 degrees
                    tracing::info!("Met a wall, turning 90 degrees right.");
                    guard.direction = match guard.direction {
                        GuardDirection::North => GuardDirection::East,
                        GuardDirection::South => GuardDirection::West,
                        GuardDirection::East => GuardDirection::South,
                        GuardDirection::West => GuardDirection::North,
                    }
                }
            },
        }
    }
}

pub fn part_one(input: &Input) -> u64 {
    let map = input.map.clone();
    let guard = input.guard.clone();

    match run_simulation(map, guard) {
        StopCondition::Exited(value) => value,
        _ => 0,
    }
}

pub fn part_two(input: &Input) -> u64 {
    return 0;
    let mut count = 0;

    let xm = input.map.keys().max_by_key(|(x, _)| x).unwrap().0;
    let ym = input.map.keys().max_by_key(|(_, y)| y).unwrap().1;

    for y in 0..=ym {
        for x in 0..=xm {
            if let Some(Tile::Floor) = input.map.get(&(x, y)) {
                // try putting a wall piece here and see if the guard loops
                let mut map = input.map.clone();
                map.insert((x, y), Tile::Floor);

                let mut guard = input.guard.clone();
            }
        }
    }
    todo!()
}

pub fn main() -> crate::Result<()> {
    let input = get_input()?;

    tracing::info!("y2024d06p01: {}", part_one(&input));
    tracing::info!("y2024d06p02: {}", part_two(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y2024d06p01() {
        let input = get_input().unwrap();
        assert_eq!(part_one(&input), 5_531);
    }

    #[test]
    fn y2024d06p02() {
        let input = get_input().unwrap();
        assert_eq!(part_two(&input), 0);
    }
}
