mod heading;
mod map_2d;

use crate::util::read_input;
use map_2d::Map2D;

fn find_best_path_score(input: &str) -> usize {
    let map = Map2D::from_string(input);
    let path = map.find_best_path_cost();

    path.unwrap()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_16/input.txt");

    find_best_path_score(&input)
}

pub fn solve_part_2() -> usize {
    let _input = read_input("src/day_16/input.txt");

    0
}

#[cfg(test)]
mod tests {
    use super::find_best_path_score;

    #[test]
    fn finds_best_path_score() {
        let input = "###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############";

        let result = find_best_path_score(input);
        assert_eq!(result, 7036);

        let input = "#################\n#...#...#...#..E#\n#.#.#.#.#.#.#.#.#\n#.#.#.#...#...#.#\n#.#.#.#.###.#.#.#\n#...#.#.#.....#.#\n#.#.#.#.#.#####.#\n#.#...#.#.#.....#\n#.#.#####.#.###.#\n#.#.#.......#...#\n#.#.###.#####.###\n#.#.#...#.....#.#\n#.#.#.#####.###.#\n#.#.#.........#.#\n#.#.#.#########.#\n#S#.............#\n#################";

        let result = find_best_path_score(input);
        assert_eq!(result, 11048);
    }
}
