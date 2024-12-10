use crate::util::Position;
use std::collections::{HashMap, HashSet};

pub struct HeightMap {
    heights: HashMap<Position, usize>,
    zero_elevations: Vec<Position>,
}

impl HeightMap {
    pub fn from_string(s: &str) -> Self {
        let mut heights = HashMap::new();
        let mut zero_elevations = Vec::new();

        s.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, character)| {
                let height = character.to_digit(10);

                match height {
                    Some(height) => {
                        let position = (x as isize, y as isize);

                        if height == 0 {
                            zero_elevations.push(position);
                        }

                        heights.insert(position, height as usize);
                    }
                    _ => {}
                }
            })
        });

        Self {
            heights,
            zero_elevations,
        }
    }
}

impl HeightMap {
    fn neighboring_positions(&self, position: &Position) -> Vec<Position> {
        let (x, y) = *position;

        vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
    }

    fn ascending_neighboring_positions(&self, position: &Position) -> Vec<(Position, usize)> {
        let current = self.heights.get(position);

        match current {
            Some(current) => {
                let neighbors = self.neighboring_positions(position);

                neighbors
                    .iter()
                    .filter_map(|neighbor| {
                        let value = self.heights.get(neighbor);
                        match value {
                            Some(value) if value == &(current + 1) => Some((*neighbor, *value)),
                            _ => None,
                        }
                    })
                    .collect()
            }
            None => vec![],
        }
    }

    fn build_trails(&self, trails: &Vec<Vec<Position>>) -> Vec<Vec<Position>> {
        trails
            .iter()
            .flat_map(|trail| {
                let last = trail.last().unwrap();
                let value = self.heights.get(last);

                if value.is_none() {
                    return vec![];
                }

                let value = value.unwrap();
                if value == &9 {
                    return vec![trail.clone()];
                }

                self.ascending_neighboring_positions(last)
                    .iter()
                    .flat_map(|(position, next)| match next == &(value + 1) {
                        true => {
                            let mut trail = trail.clone();
                            trail.push(*position);
                            self.build_trails(&vec![trail])
                        }
                        false => vec![],
                    })
                    .collect()
            })
            .collect()
    }

    fn trails_from_position(&self, position: &Position) -> Vec<Vec<Position>> {
        self.build_trails(&vec![vec![*position]])
    }

    pub fn find_trailhead_scores(&self) -> HashMap<Position, usize> {
        self.zero_elevations
            .iter()
            .map(|position| {
                let trails = self.trails_from_position(position);

                let ends: HashSet<&Position> =
                    trails.iter().map(|trail| trail.last().unwrap()).collect();

                (*position, ends.len())
            })
            .collect()
    }

    pub fn find_trailhead_trails(&self) -> HashMap<Position, usize> {
        self.zero_elevations
            .iter()
            .map(|position| (*position, self.trails_from_position(position).len()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_trails_from_position() {
        let input = "0123\n1114\n2345\n9876";
        let map = HeightMap::from_string(input);

        let result = map.trails_from_position(&(0, 0));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn finds_trails_from_position_other_map() {
        let input = "10..9..\n2...8..\n3...7..\n4567654\n...8..3\n...9..2\n.....01";
        let map = HeightMap::from_string(input);

        let result = map.trails_from_position(&(1, 0));
        assert_eq!(result.len(), 1);

        let result = map.trails_from_position(&(5, 6));
        assert_eq!(result.len(), 2);
    }
}
