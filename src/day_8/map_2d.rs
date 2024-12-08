use crate::util::Position;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct Map2D {
    nodes: HashMap<char, Vec<Position>>,
    size: (isize, isize),
}

impl Map2D {
    pub fn from_string(text: &str) -> Self {
        let mut nodes: HashMap<char, Vec<Position>> = HashMap::new();

        text.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, character)| {
                let x = x as isize;
                let y = y as isize;

                if character == '.' {
                    return;
                }

                let position = (x, y);

                let list = nodes.get_mut(&character);
                let list = match list {
                    None => Vec::from([position]),
                    Some(list) => {
                        list.push(position);
                        list.to_vec()
                    }
                };

                nodes.insert(character, list);
            });
        });

        let size_y = text.lines().count();
        let size_x = text.lines().last().unwrap().chars().count();

        Self {
            nodes,
            size: (size_x as isize, size_y as isize),
        }
    }
}

impl Map2D {
    fn is_within_map(&self, position: &Position) -> bool {
        let size = self.size;
        position.0 >= 0 && position.1 >= 0 && position.0 < size.0 && position.1 < size.1
    }

    fn antinodes_from_positions(&self, a: &Position, b: &Position) -> Vec<Position> {
        let distance_x = b.0 - a.0;
        let distance_y = b.1 - a.1;

        Vec::from([
            (a.0 - distance_x, a.1 - distance_y),
            (b.0 + distance_x, b.1 + distance_y),
        ])
    }

    fn antinodes_from_positions_with_resonance(&self, a: &Position, b: &Position) -> Vec<Position> {
        let distance_x = b.0 - a.0;
        let distance_y = b.1 - a.1;

        let mut result = Vec::new();

        [-1, 1].iter().for_each(|direction| {
            let mut index = 1;

            loop {
                let next = (
                    a.0 + index * direction * distance_x,
                    a.1 + index * direction * distance_y,
                );
                match self.is_within_map(&next) {
                    true => result.push(next),
                    false => break,
                }

                index += 1;
            }
        });

        result
    }

    fn find_antinodes_from_node_list(
        &self,
        list: &Vec<Position>,
        finder: impl Fn(&Position, &Position) -> Vec<Position>,
    ) -> HashSet<Position> {
        let mut result = HashSet::new();

        list.iter()
            .enumerate()
            .for_each(|(outer_index, outer_position)| {
                list.iter()
                    .enumerate()
                    .for_each(|(inner_index, inner_position)| {
                        if inner_index == outer_index {
                            return;
                        }

                        let nodes = finder(outer_position, inner_position);
                        result.extend(nodes.iter().filter(|position| self.is_within_map(position)));
                    });
            });

        result
    }

    pub fn find_antinodes(&self, with_resonance: bool) -> HashSet<Position> {
        let mut result = HashSet::new();

        self.nodes.iter().for_each(|(_, list)| {
            let antinodes = self.find_antinodes_from_node_list(list, |a, b| match with_resonance {
                true => self.antinodes_from_positions_with_resonance(a, b),
                false => self.antinodes_from_positions(a, b),
            });

            result.extend(antinodes.iter());
        });

        result
    }
}
