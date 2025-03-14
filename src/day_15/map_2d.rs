use crate::util::{move_steps_in_direction, Direction, Position};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MapFeature {
    Box,
    BoxLeft,
    BoxRight,
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
                    '[' => MapFeature::BoxLeft,
                    ']' => MapFeature::BoxRight,
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

    pub fn get_tiles(&self) -> &HashMap<Position, MapFeature> {
        &self.tiles
    }

    pub fn set_tiles(&mut self, tiles: HashMap<Position, MapFeature>) {
        self.tiles = tiles;
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
                    MapFeature::Empty => return Some(position),
                    MapFeature::Wall => return None,
                    _ => {}
                },
            }
        }
    }

    pub fn move_box(&mut self, from: &Position, to: &Position) {
        let from_box = self.tiles.get(from).unwrap();

        match from_box {
            MapFeature::Box => {
                self.tiles.insert(*from, MapFeature::Empty);
                self.tiles.insert(*to, MapFeature::Box);
            }
            MapFeature::BoxLeft | MapFeature::BoxRight => {}
            _ => {}
        }
    }

    pub fn sum_box_coordinates(&self) -> usize {
        let mut result = 0;

        for ((x, y), tile) in &self.tiles {
            match tile {
                MapFeature::Box | MapFeature::BoxLeft => {
                    result += ((y * 100) + x) as usize;
                }
                _ => {}
            }
        }

        result
    }
}
