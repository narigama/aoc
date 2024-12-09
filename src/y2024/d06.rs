use std::collections::{HashMap, HashSet};

use eyre::OptionExt;
use itertools::Itertools;
use rayon::prelude::*;

pub type Position = (i64, i64);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guard {
    pub position: Position,
    pub direction: Direction,
}

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

impl Guard {
    fn next(&self) -> Position {
        match self.direction {
            Direction::North => (self.position.0, self.position.1 - 1),
            Direction::East => (self.position.0 + 1, self.position.1),
            Direction::South => (self.position.0, self.position.1 + 1),
            Direction::West => (self.position.0 - 1, self.position.1),
        }
    }

    fn turn(&self) -> Direction {
        // turn right 90 degrees
        match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

pub type Input = (HashMap<Position, Tile>, Guard);

pub fn get_input() -> crate::Result<Input> {
    let input = crate::get_input(2024, 6)?;

    // setup input params
    let mut tiles = HashMap::new();
    let mut guard = None;

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.char_indices() {
            tiles.insert((x as _, y as _), Tile::from(char));

            if char == '^' {
                guard = Some(Guard { position: (x as _, y as _), direction: Direction::North })
            }
        }
    }

    let guard = guard.ok_or_eyre("There isn't a guard on the map, can't build input.")?;
    Ok((tiles, guard))
}

pub enum SimulationResult {
    OutOfBounds(u64),
    InfiniteLoop,
}

pub fn run_simulation(tiles: &HashMap<Position, Tile>, guard: &Guard) -> SimulationResult {
    let mut visited = HashSet::new();
    let mut obstructions = HashSet::new();

    let mut guard = guard.clone();

    loop {
        // mark this tile as visited
        visited.insert(guard.position);

        // look ahead
        let next_position = guard.next();

        // and do something based on what is there
        match tiles.get(&next_position) {
            None => return SimulationResult::OutOfBounds(visited.len() as _), // not on the map any more, stop
            Some(tile) => match tile {
                Tile::Floor => guard.position = next_position,
                Tile::Wall => {
                    // have we been blocked in this direction before? Then we're in a loop, stop.
                    if obstructions.contains(&(next_position, guard.direction.clone())) {
                        return SimulationResult::InfiniteLoop;
                    }

                    // otherwise mark this obstruction and turn
                    obstructions.insert((next_position, guard.direction.clone()));
                    guard.direction = guard.turn();
                }
            },
        }
    }
}

pub fn part_one(input: &Input) -> u64 {
    let (tiles, guard) = input.clone();

    match run_simulation(&tiles, &guard) {
        SimulationResult::OutOfBounds(value) => value,
        SimulationResult::InfiniteLoop => 0,
    }
}

pub fn part_two(input: &Input) -> u64 {
    // find the bounds of the map
    let (tiles, guard) = input.clone();
    let (x_max, y_max) = tiles.keys().cloned().sorted().next_back().unwrap();

    let mut obstructions = Vec::new();

    for y in 0..=y_max {
        for x in 0..=x_max {
            obstructions.push((x, y));
        }
    }

    obstructions
        .par_iter()
        .map(|(x, y)| {
            if let Some(Tile::Wall) = tiles.get(&(*x, *y)) {
                return 0; // this position is already obstructed, skip
            }

            // otherwise build a new map with an obstruction at this location
            let mut tiles = tiles.clone();
            tiles.insert((*x, *y), Tile::Wall);

            // run and count the infinite loops
            match run_simulation(&tiles, &guard) {
                SimulationResult::InfiniteLoop => 1,
                _ => 0,
            }
        })
        .sum()
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
        assert_eq!(part_two(&input), 2_165);
    }
}
