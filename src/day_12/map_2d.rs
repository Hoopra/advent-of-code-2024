use std::collections::{HashMap, HashSet};

use crate::util::{move_steps_in_direction, Direction, Position};

use super::area::Area;

pub struct Map2D {
    tiles: HashMap<Position, char>,
}

impl Map2D {
    pub fn from_string(text: &str) -> Self {
        let mut tiles: HashMap<Position, char> = HashMap::new();

        text.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, character)| {
                let x = x as isize;
                let y = y as isize;

                let position = (x, y);

                tiles.insert(position, character);
            });
        });

        Self { tiles }
    }
}

impl Map2D {
    pub fn find_areas(&self) -> Vec<Area> {
        let mut used: HashSet<Position> = HashSet::new();
        let mut result: Vec<Area> = vec![];

        for (position, character) in self.tiles.iter() {
            if used.contains(position) {
                continue;
            }

            let area = self.find_area_from(position, character);

            used.extend(area.tiles.clone());
            result.push(area);
        }

        result
    }

    pub fn find_area_from(&self, position: &Position, character: &char) -> Area {
        let mut checked = HashSet::new();
        let mut pending: Vec<Position> = vec![position.clone()];

        let mut tiles: HashSet<Position> = HashSet::new();
        let mut perimeter: HashSet<Position> = HashSet::new();

        while let Some(next) = pending.pop() {
            checked.insert(next);

            match self.tiles.get(&next) {
                None => {
                    perimeter.insert(next.clone());
                }
                Some(next_character) => match next_character == character {
                    false => {
                        perimeter.insert(next.clone());
                    }
                    true => {
                        tiles.insert(next.clone());

                        for direction in Direction::cardinal() {
                            let neighbor = move_steps_in_direction(&next, 1, &direction);
                            if checked.contains(&neighbor) {
                                continue;
                            }

                            pending.push(neighbor);
                        }
                    }
                },
            }
        }

        Area::new(*character, tiles, perimeter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_specific_area_fence_cost() {
        let input = "AAAA\nBBCD\nBBCC\nEEEC";

        let area = Map2D::from_string(input).find_area_from(&(2, 2), &'C');

        assert_eq!(area.tiles.len(), 4);
        assert_eq!(area.perimeter_tiles.len(), 8);
        assert_eq!(area.fence_cost(), 40);
    }

    // #[test]
    // fn finds_specific_area_fence_cost_with_discount() {
    //     let input = "AAAA\nBBCD\nBBCC\nEEEC";

    //     let area = Map2D::from_string(input).find_area_from(&(2, 2), &'C');

    //     assert_eq!(area.tiles.len(), 4);
    //     assert_eq!(area.number_of_sides(), 8);
    //     assert_eq!(area.fence_cost_with_discount(), 32);
    // }

    #[test]
    fn find_areas() {
        let input = "AAAA\nBBCD\nBBCC\nEEEC";

        let areas = Map2D::from_string(input).find_areas();

        let area_a = areas.iter().find(|area| area.character == 'A').unwrap();
        assert_eq!(area_a.tiles.len(), 4);
        assert_eq!(area_a.fence_cost(), 40);

        let area_b = areas.iter().find(|area| area.character == 'B').unwrap();
        assert_eq!(area_b.tiles.len(), 4);
        assert_eq!(area_b.perimeter_tiles.len(), 8);
        assert_eq!(area_b.fence_cost(), 32);

        let area_c = areas.iter().find(|area| area.character == 'C').unwrap();
        assert_eq!(area_c.tiles.len(), 4);
        assert_eq!(area_c.perimeter_tiles.len(), 8);
        assert_eq!(area_c.fence_cost(), 40);

        let area_d = areas.iter().find(|area| area.character == 'D').unwrap();
        assert_eq!(area_d.fence_cost(), 4);

        let area_e = areas.iter().find(|area| area.character == 'E').unwrap();
        assert_eq!(area_e.fence_cost(), 24);
    }
}
