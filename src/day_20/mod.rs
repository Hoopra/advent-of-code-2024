mod map_2d;
mod path;

use crate::util::{read_input, Position};
use map_2d::Map2D;

fn find_best_cheats(input: &str, minimum_saved: usize) -> usize {
    find_cheats(input)
        .iter()
        .filter(|(position, saved)| {
            println!("cheat at {:?} saves {}", position, saved);
            saved >= &minimum_saved
        })
        .count()
}

fn find_cheats(input: &str) -> Vec<(Position, usize)> {
    let map = Map2D::from_string(input);
    let path_base = map.best_path().unwrap().len();

    let cheats = map.possible_cheats();

    cheats
        .iter()
        .enumerate()
        .filter_map(|(i, position)| {
            println!("{} / {}", i, cheats.len());
            let length = map.path_length_with_cheat(&position);
            match length < path_base {
                true => Some((*position, path_base - length)),
                _ => None,
            }
        })
        .collect()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_20/input.txt");

    // TODO: Optimise
    find_best_cheats(&input, 100)
}

pub fn solve_part_2() -> usize {
    let _input = read_input("src/day_20/input.txt");

    0
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn finds_cheats_in_map() {
        let input = "###############\n#...#...#.....#\n#.#.#.#.#.###.#\n#S#...#.#.#...#\n#######.#.#.###\n#######.#.#...#\n#######.#.###.#\n###..E#...#...#\n###.#######.###\n#...###...#...#\n#.#####.#.###.#\n#.#...#.#.#...#\n#.#.#.#.#.#.###\n#...#...#...###\n###############";

        let result = find_cheats(input);

        let grouped = result
            .iter()
            .fold(HashMap::new(), |mut result, (_, saved)| {
                let current = result.get(saved).unwrap_or(&0);
                result.insert(*saved, current + 1);

                result
            });

        assert_eq!(
            grouped,
            HashMap::from([
                (2, 14),
                (4, 14),
                (6, 2),
                (8, 4),
                (10, 2),
                (12, 3),
                (20, 1),
                (36, 1),
                (38, 1),
                (40, 1),
                (64, 1),
            ])
        );
    }

    #[test]
    fn finds_cheats_with_minimum_saving() {
        let input = "###############\n#...#...#.....#\n#.#.#.#.#.###.#\n#S#...#.#.#...#\n#######.#.#.###\n#######.#.#...#\n#######.#.###.#\n###..E#...#...#\n###.#######.###\n#...###...#...#\n#.#####.#.###.#\n#.#...#.#.#...#\n#.#.#.#.#.#.###\n#...#...#...###\n###############";

        let result = find_best_cheats(input, 20);
        assert_eq!(result, 5);
    }
}
