use std::collections::HashMap;

type Position = (isize, isize);

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn all() -> [Direction; 8] {
        [
            Direction::N,
            Direction::NE,
            Direction::E,
            Direction::SE,
            Direction::S,
            Direction::SW,
            Direction::W,
            Direction::NW,
        ]
    }

    pub fn corner() -> [Direction; 4] {
        [Direction::NE, Direction::SE, Direction::SW, Direction::NW]
    }
}

impl Direction {
    pub fn step_2d(&self) -> (isize, isize) {
        match self {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }
}

struct WordBounds {
    size: (isize, isize),
    entries: HashMap<Position, char>,
}

impl WordBounds {
    pub fn from_text(text: &str) -> Self {
        let entries = text
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, character)| ((x as isize, y as isize), character))
            })
            .collect();

        let size_y = text.lines().count() as isize;
        let size_x = text.lines().nth(0).unwrap().chars().count() as isize;

        Self {
            entries,
            size: (size_x, size_y),
        }
    }
}

impl WordBounds {
    pub fn find_instances(&self, match_character: char) -> Vec<Position> {
        let mut result = Vec::new();

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let character = self.entries.get(&(x, y));
                match character {
                    Some(character) if character == &match_character => {
                        result.push((x, y));
                    }
                    _ => {}
                }
            }
        }

        result
    }

    pub fn find_word_any_direction(&self, word: &str, position: &Position) -> u32 {
        let directions = Direction::all();

        directions.iter().fold(0, |result, direction| {
            let found = self.find_word_in_direction(word, position, direction);

            match found {
                true => result + 1,
                _ => result,
            }
        })
    }

    pub fn find_halfway_points_cross(
        &self,
        word: &str,
        position: &Position,
        steps: isize,
    ) -> Vec<Position> {
        let directions = Direction::corner();

        directions
            .iter()
            .filter_map(|direction| {
                let found = self.find_word_in_direction(word, position, direction);

                match found {
                    false => None,
                    true => {
                        let step = direction.step_2d();
                        Some((position.0 + steps * step.0, position.1 + steps * step.1))
                    }
                }
            })
            .collect()
    }

    fn find_word_in_direction(
        &self,
        word: &str,
        position: &Position,
        direction: &Direction,
    ) -> bool {
        let word_length = word.len();
        let step = direction.step_2d();
        let mut search_index = 0;
        let mut current_position = *position;

        while let Some(found) = self.entries.get(&current_position) {
            let matcher = word.chars().nth(search_index).unwrap();

            if found != &matcher {
                return false;
            }

            if search_index == word_length - 1 {
                return true;
            }

            search_index += 1;
            let new_x = current_position.0 + step.0;
            let new_y = current_position.1 + step.1;
            current_position = (new_x, new_y);
        }

        false
    }
}

pub fn find_instances(text: String, matcher: &str) -> u32 {
    let starting_letter = matcher.chars().nth(0).unwrap();

    let bounds = WordBounds::from_text(&text);
    let positions = bounds.find_instances(starting_letter);

    positions.iter().fold(0, |result, (x, y)| {
        result + bounds.find_word_any_direction(matcher, &(*x, *y))
    })
}

pub fn find_cross_instances(text: String, matcher: &str) -> u32 {
    let starting_letter = matcher.chars().nth(0).unwrap();
    let halfway_index = (0.5 * (matcher.len() as f64)).floor() as isize;

    let bounds = WordBounds::from_text(&text);
    let positions = bounds.find_instances(starting_letter);

    let halfway_points: Vec<(isize, isize)> = positions
        .iter()
        .flat_map(|(x, y)| {
            let positions = bounds.find_halfway_points_cross(matcher, &(*x, *y), halfway_index);

            positions
        })
        .collect();

    let mut cross_map: HashMap<Position, u32> = HashMap::new();

    halfway_points.iter().for_each(|position| {
        let current = cross_map.get(position).unwrap_or(&0);
        cross_map.insert(*position, current + 1);
    });

    cross_map.iter().fold(0, |sum, (_, value)| match value {
        2 => sum + 1,
        _ => sum,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_word_in_direction() {
        let sample = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        let searcher = WordBounds::from_text(sample);

        let result = searcher.find_word_in_direction("XMAS", &(3, 9), &Direction::NE);
        assert_eq!(result, true);

        let result = searcher.find_word_in_direction("XMAS", &(3, 9), &Direction::NW);
        assert_eq!(result, true);
    }
}
