use crate::util::{move_steps_in_direction, Direction, Position};
use std::collections::HashSet;

type EdgeSegment = (Position, Direction);

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

fn distance_between_segments(a: &EdgeSegment, b: &EdgeSegment) -> isize {
    let position_a = a.0;
    let position_b = b.0;

    ((position_a.0 - position_b.0) + (position_a.1 - position_b.1)).abs()
}

fn sort_segments_by_distance(
    all_segments: HashSet<EdgeSegment>,
    to: &EdgeSegment,
) -> Vec<EdgeSegment> {
    let mut result = vec![];
    for segment in all_segments {
        result.push(segment);
    }

    result.sort_by(|a, b| {
        let distance_a = distance_between_segments(a, to);
        let distance_b = distance_between_segments(b, to);

        distance_a.cmp(&distance_b)
    });

    result
}

fn are_segments_adjacent(a: &EdgeSegment, b: &EdgeSegment) -> bool {
    if a.1 != b.1 {
        return false;
    }

    let position_a = a.0;
    let position_b = b.0;

    let diff_x = position_a.0 - position_b.0;
    let diff_y = position_a.1 - position_b.1;

    match a.1 {
        Direction::E | Direction::W => diff_y.abs() == 1 && diff_x == 0,
        _ => diff_x.abs() == 1 && diff_y == 0,
    }
}

fn find_adjacent_segments(
    segment: &EdgeSegment,
    all_segments: &HashSet<EdgeSegment>,
) -> HashSet<EdgeSegment> {
    let sorted = sort_segments_by_distance(all_segments.clone(), segment);
    let mut adjacent_segments: HashSet<EdgeSegment> = HashSet::from([*segment]);

    for segment in sorted {
        if adjacent_segments.contains(&segment) {
            continue;
        }

        let adjacent = adjacent_segments
            .iter()
            .find(|test| test != &&segment && are_segments_adjacent(&segment, test));

        if adjacent.is_some() {
            adjacent_segments.insert(segment);
        }
    }

    adjacent_segments
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

    fn segments(&self) -> HashSet<EdgeSegment> {
        let mut result = HashSet::new();

        self.tiles.iter().for_each(|tile| {
            Direction::cardinal().iter().for_each(|direction| {
                let neighbor = move_steps_in_direction(tile, 1, direction);
                if self.tiles.contains(&neighbor) {
                    return;
                }

                result.insert((*tile, *direction));
            });
        });

        result
    }

    pub fn number_of_sides(&self) -> usize {
        let segments = self.segments();

        let mut edges: HashSet<EdgeSegment> = HashSet::new();
        let mut used: HashSet<EdgeSegment> = HashSet::new();

        for segment in &segments {
            if used.contains(&segment) {
                continue;
            }

            let adjacent = find_adjacent_segments(segment, &segments);

            edges.insert(*segment);

            used.insert(*segment);
            used.extend(adjacent);
        }

        edges.len()
    }

    pub fn fence_cost_with_discount(&self) -> usize {
        self.number_of_sides() * self.tiles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_if_segments_are_adjacent() {
        let segment_a = ((2, 1), Direction::W);
        let segment_b = ((2, 2), Direction::W);
        let segment_c = ((2, 1), Direction::N);
        let segment_d = ((3, 1), Direction::N);

        assert_eq!(are_segments_adjacent(&segment_a, &segment_b), true);
        assert_eq!(are_segments_adjacent(&segment_a, &segment_c), false);
        assert_eq!(are_segments_adjacent(&segment_c, &segment_d), true);
        assert_eq!(are_segments_adjacent(&segment_b, &segment_d), false);
    }

    #[test]
    fn finds_adjacent_segments_in_area() {
        let tiles = HashSet::from([(2, 1), (2, 2), (3, 3), (3, 2)]);
        let perimeter_tiles = HashSet::from([
            (3, 1),
            (4, 3),
            (2, 0),
            (2, 3),
            (1, 2),
            (1, 1),
            (3, 4),
            (4, 2),
        ]);

        let area = Area {
            character: 'C',
            tiles,
            perimeter_tiles,
        };

        let segment = ((2, 1), Direction::W);
        let adjacent = find_adjacent_segments(&segment, &area.segments());

        assert_eq!(
            adjacent,
            HashSet::from([((2, 1), Direction::W), ((2, 2), Direction::W)])
        );
    }

    #[test]
    fn finds_all_adjacent_segments() {
        let segments = HashSet::from([
            ((2, 1), Direction::W),
            ((2, 3), Direction::W),
            ((2, 2), Direction::W),
            ((2, 4), Direction::W),
            ((2, 5), Direction::W),
        ]);

        let segment = ((2, 1), Direction::W);

        let adjacent = find_adjacent_segments(&segment, &segments);

        assert_eq!(adjacent, segments);
    }
}
