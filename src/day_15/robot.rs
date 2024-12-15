use crate::util::{move_steps_in_direction, Direction, Position};

use super::map_2d::{Map2D, MapFeature};

pub struct Robot {
    position: Position,
    moves: Vec<Direction>,
}

impl Robot {
    pub fn from_moves(position: Position, input: &str) -> Self {
        let mut moves: Vec<Direction> = vec![];

        input.lines().for_each(|line| {
            line.chars().for_each(|character| {
                let direction = match character {
                    '<' => Direction::W,
                    '>' => Direction::E,
                    'v' => Direction::S,
                    _ => Direction::N,
                };

                moves.push(direction);
            });
        });

        Self { position, moves }
    }
}

impl Robot {
    pub fn move_in_map(&mut self, map: &mut Map2D) {
        for direction in &self.moves {
            let new_position = move_steps_in_direction(&self.position, 1, direction);
            match map.get(&new_position) {
                None => {}
                Some(tile) => match tile {
                    MapFeature::Empty => self.position = new_position,
                    MapFeature::Box => {
                        let non_box = map.find_empty_in_direction(&new_position, direction);
                        match non_box {
                            None => {}
                            Some(empty_position) => {
                                map.move_box(&new_position, &empty_position);
                                self.position = new_position;
                            }
                        }
                    }
                    MapFeature::Wall => {}
                },
            }
        }
    }
}
