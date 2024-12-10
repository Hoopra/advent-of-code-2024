use crate::util::{move_steps_in_direction, Direction, Position};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
enum MapFeature {
    Empty,
    Obstacle,
}

#[derive(Clone)]
pub struct Map2D {
    tiles: HashMap<Position, MapFeature>,
    start: Position,
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
                    '#' => MapFeature::Obstacle,
                    '^' => {
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
    pub fn step_once(&self, position: &Position, heading: &Direction) -> (Position, Direction) {
        let new_position = move_steps_in_direction(position, 1, heading);
        let next = self.tiles.get(&new_position);

        match next {
            None => (new_position, *heading),
            Some(feature) => match feature {
                MapFeature::Empty => (new_position, *heading),
                MapFeature::Obstacle => {
                    let new_heading = heading.rotate_90_degress_clockwise();
                    self.step_once(position, &new_heading)
                }
            },
        }
    }

    pub fn steps_to_exit(&self) -> HashSet<Position> {
        let mut heading = Direction::N;
        let mut position = self.start.clone();
        let mut positions = HashSet::new();

        while let Some(_) = self.tiles.get(&position) {
            positions.insert(position);
            (position, heading) = self.step_once(&position, &heading);
        }

        positions
    }

    pub fn obstacles_resulting_in_loop(&self) -> usize {
        let steps = self.steps_to_exit();

        steps
            .into_iter()
            .filter(|step| self.is_loop_with_obstacle(*step))
            .count()
    }

    pub fn is_loop(&self) -> bool {
        let mut heading = Direction::N;
        let mut position = self.start.clone();
        let mut positions: HashSet<(Position, Direction)> = HashSet::new();

        while let Some(_) = self.tiles.get(&position) {
            positions.insert((position, heading));
            (position, heading) = self.step_once(&position, &heading);

            if positions.contains(&(position, heading)) {
                return true;
            }
        }

        false
    }

    pub fn is_loop_with_obstacle(&self, obstacle: Position) -> bool {
        let mut new_map = self.clone();
        new_map.tiles.insert(obstacle, MapFeature::Obstacle);

        new_map.is_loop()
    }
}
