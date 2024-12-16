use super::heading::Heading;
use crate::util::{move_steps_in_direction, Direction, Position};
use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq)]
enum MapFeature {
    Empty,
    Wall,
}

pub struct Map2D {
    tiles: HashMap<Position, MapFeature>,
    start: Position,
    end: Position,
}

impl Map2D {
    pub fn from_string(text: &str) -> Self {
        let mut tiles: HashMap<Position, MapFeature> = HashMap::new();
        let mut start: Position = (0, 0);
        let mut end: Position = (0, 0);

        text.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, character)| {
                let x = x as isize;
                let y = y as isize;

                let position = (x, y);
                let tile = match character {
                    '#' => MapFeature::Wall,
                    'S' => {
                        start = position;
                        MapFeature::Empty
                    }
                    'E' => {
                        end = position;
                        MapFeature::Empty
                    }
                    _ => MapFeature::Empty,
                };

                tiles.insert(position, tile);
            });
        });

        Self { tiles, start, end }
    }
}

impl Map2D {
    fn possible_steps_from(
        &self,
        position: &Position,
        direction: &Direction,
    ) -> Vec<(Position, Direction)> {
        let directions = vec![
            direction.rotate_90_degress_counter_clockwise(),
            *direction,
            direction.rotate_90_degress_clockwise(),
        ];

        directions
            .iter()
            .filter_map(|direction| {
                let next = move_steps_in_direction(position, 1, direction);
                match self.tiles.get(&next) {
                    Some(tile) if tile == &MapFeature::Empty => Some((next, *direction)),
                    _ => None,
                }
            })
            .collect()
    }

    pub fn find_best_path_cost(&self) -> Option<usize> {
        let mut queue: BinaryHeap<Heading> = BinaryHeap::from([(self.start, Direction::E).into()]);
        let mut best_score_at: HashMap<Position, usize> = HashMap::from([(self.start, 0)]);

        while let Some(next) = queue.pop() {
            let Heading {
                position,
                direction,
                ..
            } = next;

            if position == self.end {
                return Some(next.cost());
            }

            let possible = self.possible_steps_from(&position, &direction);

            for (new_position, new_direction) in possible {
                let next_heading = next.new_with(new_position, new_direction);

                let score = next_heading.cost();
                let best_score = best_score_at
                    .get(&new_position)
                    .unwrap_or(&usize::max_value());

                if &score > best_score {
                    break;
                }

                best_score_at.insert(position, score);
                queue.push(next_heading);
            }
        }

        None
    }
}
