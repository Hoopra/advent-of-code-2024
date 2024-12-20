use crate::util::{move_steps_in_direction, Direction, Position};
use std::{
    collections::{BinaryHeap, HashMap},
    isize,
};

use super::path::Path;

#[derive(Debug, PartialEq)]
enum MapFeature {
    Empty,
    Obstructed,
}

pub struct Map2D {
    tiles: HashMap<Position, MapFeature>,
    size: (isize, isize),
}

pub fn coordinate_from_string(input: &str) -> Position {
    let mut components = input.split(',');
    let x: isize = components.nth(0).unwrap().parse().unwrap();
    let y: isize = components.nth(0).unwrap().parse().unwrap();

    (x, y)
}

impl Map2D {
    pub fn from_string(input: &str, size: (isize, isize), add_walls: usize) -> Self {
        let mut tiles = HashMap::new();

        for x in 0..=size.0 {
            for y in 0..=size.1 {
                tiles.insert((x, y), MapFeature::Empty);
            }
        }

        input.lines().take(add_walls).for_each(|line| {
            let position = coordinate_from_string(line);

            tiles.insert(position, MapFeature::Obstructed);
        });

        Self { tiles, size }
    }
}

fn construct_path(
    path_map: &HashMap<Position, Position>,
    start: (isize, isize),
    end: (isize, isize),
) -> Vec<Position> {
    let mut result = vec![];

    let mut next = &end;

    while next != &start {
        result.push(next.clone());
        next = path_map.get(next).unwrap();
    }

    result
}

impl Map2D {
    pub fn add_wall_from_string(&mut self, input: &str) {
        let position = coordinate_from_string(input);
        self.tiles.insert(position, MapFeature::Obstructed);
    }

    fn possible_steps_from(&self, position: &Position) -> Vec<Position> {
        [Direction::E, Direction::S, Direction::W, Direction::N]
            .iter()
            .filter_map(|direction| {
                let next = move_steps_in_direction(position, 1, direction);
                match self.tiles.get(&next) {
                    None => None,
                    Some(feature) => match feature {
                        MapFeature::Empty => Some(next),
                        _ => None,
                    },
                }
            })
            .collect()
    }

    pub fn best_path(&self) -> Option<Vec<Position>> {
        let start = (0, 0);
        let end = self.size;

        let mut distances: HashMap<Position, isize> = HashMap::from([(start, 0)]);
        let mut came_from: HashMap<Position, Position> = HashMap::new();
        let mut queue: BinaryHeap<Path> = BinaryHeap::from([Path::new(start, 0)]);

        distances.insert(start, 0);

        while let Some(path) = queue.pop() {
            let Path { position, .. } = path;

            for neighbor in self.possible_steps_from(&position) {
                if neighbor == end {
                    came_from.insert(end, position);

                    let path = construct_path(&came_from, start, end);
                    return Some(path);
                }

                let new_distance = distances.get(&position).unwrap() + 1;
                let distance = distances.get(&neighbor).unwrap_or(&isize::MAX);

                if &new_distance >= distance {
                    continue;
                }

                distances.insert(neighbor, new_distance);
                came_from.insert(neighbor, position);
                queue.push(Path::new(neighbor, new_distance));
            }
        }

        None
    }
}
