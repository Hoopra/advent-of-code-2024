use crate::util::{move_steps_in_direction, Direction, Position};

use super::map_2d::{Map2D, MapFeature};

pub struct Robot {
    pub position: Position,
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

fn move_boxes(map: &mut Map2D, new_position: &Position, direction: &Direction) -> bool {
    let non_box = map.find_empty_in_direction(new_position, direction);
    match non_box {
        None => false,
        Some(empty_position) => {
            map.move_box(&new_position, &empty_position);
            true
        }
    }
}

fn move_large_boxes(map: &mut Map2D, new_position: &Position, direction: &Direction) -> bool {
    match direction {
        Direction::E | Direction::W => move_large_boxes_x(map, new_position, direction),
        _ => move_large_boxes_y(map, new_position, direction),
    }
}

fn move_large_boxes_x(map: &mut Map2D, position: &Position, direction: &Direction) -> bool {
    let mut tiles = map.get_tiles().clone();

    let mut boxes_to_move: Vec<Position> = vec![*position];
    let mut boxes: Vec<Position> = vec![*position];

    while let Some(next) = boxes.pop() {
        let move_to = match direction {
            Direction::W => (next.0 - 1, next.1),
            _ => (next.0 + 2, next.1),
        };

        match map.get(&move_to).unwrap() {
            MapFeature::Wall => {
                return false;
            }
            MapFeature::BoxLeft => {
                boxes.push(move_to);
                boxes_to_move.push(move_to);
            }
            MapFeature::BoxRight => {
                let new_position = (move_to.0 - 1, move_to.1);

                boxes.push(new_position);
                boxes_to_move.push(new_position);
            }
            _ => {}
        };
    }

    let mut moved: Vec<Position> = vec![];

    for next in boxes_to_move.iter().rev() {
        if moved.contains(next) {
            continue;
        }

        moved.push(*next);

        match direction {
            Direction::E => {
                tiles.insert(*next, MapFeature::Empty);
                tiles.insert((next.0 + 1, next.1), MapFeature::BoxLeft);
                tiles.insert((next.0 + 2, next.1), MapFeature::BoxRight);
            }
            _ => {
                tiles.insert((next.0 + 1, next.1), MapFeature::Empty);
                tiles.insert((next.0 - 1, next.1), MapFeature::BoxLeft);
                tiles.insert(*next, MapFeature::BoxRight);
            }
        }
    }

    map.set_tiles(tiles);

    true
}

fn box_positions(left: Position) -> [(MapFeature, Position); 2] {
    let positions = [
        (MapFeature::BoxLeft, left),
        (MapFeature::BoxRight, (left.0 + 1, left.1)),
    ];

    positions
}

fn move_large_boxes_y(map: &mut Map2D, position: &Position, direction: &Direction) -> bool {
    let mut tiles = map.get_tiles().clone();

    let mut boxes: Vec<Position> = vec![*position];
    let mut boxes_to_move: Vec<Position> = vec![*position];

    while let Some(next) = boxes.pop() {
        for (_, box_position) in box_positions(next) {
            let move_to = move_steps_in_direction(&box_position, 1, direction);

            match map.get(&move_to).unwrap() {
                MapFeature::Wall => return false,
                MapFeature::BoxLeft => {
                    boxes.push(move_to);
                    boxes_to_move.push(move_to);
                }
                MapFeature::BoxRight => {
                    let new_position = (move_to.0 - 1, move_to.1);

                    boxes.push(new_position);
                    boxes_to_move.push(new_position);
                }
                _ => {}
            };
        }

        let mut moved: Vec<Position> = vec![];

        for next in boxes_to_move.iter().rev() {
            if moved.contains(next) {
                continue;
            }

            moved.push(*next);
            for (feature, box_position) in box_positions(*next) {
                let move_to = move_steps_in_direction(&box_position, 1, direction);

                tiles.insert(box_position, MapFeature::Empty);
                tiles.insert(move_to, feature);
            }
        }
    }

    map.set_tiles(tiles);

    true
}

impl Robot {
    pub fn move_in_map(&mut self, map: &mut Map2D) {
        for direction in &self.moves {
            let new_position = move_steps_in_direction(&self.position, 1, direction);

            let result = match map.get(&new_position) {
                None => false,
                Some(tile) => match tile {
                    MapFeature::Empty => true,
                    MapFeature::Box => move_boxes(map, &new_position, direction),
                    MapFeature::BoxLeft => move_large_boxes(map, &new_position, direction),
                    MapFeature::BoxRight => {
                        move_large_boxes(map, &(new_position.0 - 1, new_position.1), direction)
                    }
                    MapFeature::Wall => false,
                },
            };

            if result {
                self.position = new_position;
            }
        }
    }
}
