use crate::util::Position;
use std::collections::HashMap;

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
                let height = character.to_digit(10).unwrap() as usize;

                let position = (x as isize, y as isize);

                if height == 0 {
                    zero_elevations.push(position);
                }

                heights.insert(position, height);
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

    fn build_trails(&self, trails: &Vec<Vec<Position>>) -> Vec<Vec<Position>> {
        trails
            .iter()
            .flat_map(|trail| {
                let last = trail.last().unwrap();
                let value = self.heights.get(last);

                println!("last {:?}, value {:?}", last, value);

                if value.is_none() {
                    return vec![];
                }

                let value = value.unwrap();
                if value == &9 {
                    return vec![trail.clone()];
                }

                self.neighboring_positions(last)
                    .iter()
                    .filter_map(|position| {
                        if trail.contains(position) {
                            return None;
                        }

                        let next = self.heights.get(position);
                        match next {
                            None => None,
                            Some(next) => Some((position, next)),
                        }
                    })
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

    pub fn find_trails(&self) -> HashMap<Position, usize> {
        let mut result = HashMap::new();

        self.zero_elevations.iter().for_each(|position| {
            let trails = self.build_trails(&vec![vec![*position]]);

            result.insert(*position, trails.len());
        });

        result
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::HeightMap;

    #[test]
    fn finds_trails_from_starting_positions() {
        let input = "0123\n1234\n8765\n9876";
        let map = HeightMap::from_string(input);

        let trails = map.find_trails();

        println!("trails: {:?}", trails);

        assert_eq!(trails, HashMap::from([((0, 0), 3)]));
    }
}
