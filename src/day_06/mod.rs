mod map_2d;

use map_2d::Map2D;

use crate::util::read_input;

fn count_guard_steps(input: &str) -> usize {
    let map = Map2D::from_string(input);

    let steps = map.steps_to_exit();

    steps.len()
}

fn count_obstacles_for_loop(input: &str) -> usize {
    let map = Map2D::from_string(input);

    map.obstacles_resulting_in_loop()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_06/input.txt");

    count_guard_steps(&input)
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_06/input.txt");

    // TODO: optimise
    count_obstacles_for_loop(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_guard_steps() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";

        let result = count_guard_steps(input);

        assert_eq!(result, 41);
    }

    #[test]
    fn counts_obstacles_for_loop() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let result = count_obstacles_for_loop(input);

        assert_eq!(result, 6);
    }
}
