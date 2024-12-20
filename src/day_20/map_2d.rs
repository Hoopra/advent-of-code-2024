use crate::util::{move_steps_in_direction, Direction, Position};
use std::{
    collections::{BinaryHeap, HashMap},
    isize,
};

use super::path::Path;

#[derive(Debug, PartialEq, Clone)]
enum MapFeature {
    Empty,
    Wall,
}

#[derive(Clone)]
pub struct Map2D {
    tiles: HashMap<Position, MapFeature>,
    start: Position,
    end: Position,
    size: Position,
}

impl Map2D {
    pub fn from_string(text: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        text.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, character)| {
                let x = x as isize;
                let y = y as isize;
                let position = (x, y);

                let feature = match character {
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

                tiles.insert(position, feature);
            });
        });

        let size_y = text.lines().count();
        let size_x = text.lines().nth(0).unwrap().chars().count();

        Self {
            tiles,
            start,
            end,
            size: (size_x as isize, size_y as isize),
        }
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

fn positions_form_line(reference: &Position, positions: &Vec<Position>) -> bool {
    let (x, y) = reference;

    let line_x = positions.iter().all(|position| &position.0 == x);
    let line_y = positions.iter().all(|position| &position.1 == y);

    line_x || line_y
}

impl Map2D {
    pub fn possible_cheats(&self) -> Vec<Position> {
        self.tiles
            .iter()
            .filter_map(|(position, feature)| match feature {
                MapFeature::Empty => return None,
                _ => {
                    let (x, y) = position;
                    let (x_end, y_end) = &self.size;

                    if x == &0 || y == &0 || x == x_end || y == y_end {
                        return None;
                    }

                    let neighbors = self.possible_steps_from(&position);

                    match neighbors.len() >= 2 {
                        true => Some(*position),
                        _ => None,
                    }
                }
            })
            .collect()
    }

    fn remove_wall(&mut self, position: &Position) {
        self.tiles.insert(*position, MapFeature::Empty);
    }

    pub fn path_length_with_cheat(&self, cheat: &Position) -> usize {
        let mut map = self.clone();
        map.remove_wall(&cheat);

        map.best_path().unwrap().len()
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
        let Map2D { start, end, .. } = *self;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_path_length_with_cheat() {
        let input = "###############\n#...#...#.....#\n#.#.#.#.#.###.#\n#S#...#.#.#...#\n#######.#.#.###\n#######.#.#...#\n#######.#.###.#\n###..E#...#...#\n###.#######.###\n#...###...#...#\n#.#####.#.###.#\n#.#...#.#.#...#\n#.#.#.#.#.#.###\n#...#...#...###\n###############";
        let map = Map2D::from_string(input);

        let result = map.path_length_with_cheat(&(8, 1));
        assert_eq!(result, 72);

        let result = map.path_length_with_cheat(&(10, 7));
        assert_eq!(result, 64);

        let result = map.path_length_with_cheat(&(8, 8));
        assert_eq!(result, 46);

        let result = map.path_length_with_cheat(&(6, 7));
        assert_eq!(result, 20);
    }
}
