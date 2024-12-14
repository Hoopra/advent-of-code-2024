use crate::util::{move_steps_in_direction, Direction, Position};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub character: char,
    pub tiles: HashSet<Position>,
    pub perimeter_tiles: HashSet<Position>,
}

impl Area {
    pub fn new(
        character: char,
        tiles: HashSet<Position>,
        perimeter_tiles: HashSet<Position>,
    ) -> Self {
        Self {
            character,
            tiles,
            perimeter_tiles,
        }
    }
}

impl Area {
    fn calculate_perimeter(&self) -> usize {
        let mut perimeter = 0;

        for tile in &self.perimeter_tiles {
            for direction in Direction::cardinal() {
                let position = move_steps_in_direction(tile, 1, &direction);
                if self.tiles.contains(&position) {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }

    pub fn fence_cost(&self) -> usize {
        self.calculate_perimeter() * self.tiles.len()
    }

    fn segments(&self) -> HashMap<(Position, Direction), bool> {
        let mut result = HashMap::new();

        self.tiles.iter().for_each(|tile| {
            Direction::cardinal().iter().for_each(|direction| {
                let neighbor = move_steps_in_direction(tile, 1, direction);
                if self.tiles.contains(&neighbor) {
                    return;
                }

                result.insert((*tile, *direction), true);
            });
        });

        result
    }

    pub fn number_of_sides(&self) -> usize {
        let mut segments = self.segments();

        for segment in segments {
            //
        }

        0
    }

    pub fn fence_cost_with_discount(&self) -> usize {
        self.number_of_sides() * self.tiles.len()
    }
}
