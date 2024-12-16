use crate::util::{move_steps_in_direction, Direction, Position};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum MapFeature {
    Box,
    Empty,
    Wall,
}

pub struct Map2D {
    tiles: HashMap<Position, MapFeature>,
    pub start: Position,
}

impl Map2D {
    pub fn from_string(text: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut start = (0, 0);

        text.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, character)| {
                let x = x as isize;
                let y = y as isize;

                let feature = match character {
                    '#' => MapFeature::Wall,
                    'O' => MapFeature::Box,
                    '@' => {
                        start = (x, y);
                        MapFeature::Empty
                    }
                    _ => MapFeature::Empty,
                };

                tiles.insert((x, y), feature);
            });
        });

        Self { tiles, start }
    }
}

impl Map2D {
    pub fn get(&self, position: &Position) -> Option<&MapFeature> {
        self.tiles.get(position)
    }

    pub fn find_empty_in_direction(
        &self,
        position: &Position,
        direction: &Direction,
    ) -> Option<Position> {
        let mut position = *position;

        loop {
            position = move_steps_in_direction(&position, 1, direction);
            let tile = self.get(&position);

            match tile {
                None => return None,
                Some(some) => match some {
                    MapFeature::Box => {}
                    MapFeature::Empty => return Some(position),
                    MapFeature::Wall => return None,
                },
            }
        }
    }

    pub fn move_box(&mut self, from: &Position, to: &Position) {
        self.tiles.insert(*from, MapFeature::Empty);
        self.tiles.insert(*to, MapFeature::Box);
    }

    pub fn sum_box_coordinates(&self) -> usize {
        let mut result = 0;

        for ((x, y), tile) in &self.tiles {
            match tile {
                MapFeature::Box => {
                    result += ((y * 100) + x) as usize;
                }
                _ => {}
            }
        }

        result
    }
}
