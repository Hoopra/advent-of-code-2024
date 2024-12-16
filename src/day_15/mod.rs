mod map_2d;
mod robot;

use crate::util::read_input;
use map_2d::Map2D;
use robot::Robot;

fn determine_arrangement_after_moves(input: &str) -> Map2D {
    let mut components = input.split("\n\n");

    let mut map = Map2D::from_string(components.nth(0).unwrap());
    let mut robot = Robot::from_moves(map.start, components.nth(0).unwrap());

    robot.move_in_map(&mut map);

    map
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_15/input.txt");

    let map = determine_arrangement_after_moves(&input);
    map.sum_box_coordinates()
}

pub fn solve_part_2() -> usize {
    let _input = read_input("src/day_15/input.txt");

    0
}

#[cfg(test)]
mod tests {
    use map_2d::MapFeature;

    use super::*;

    #[test]
    fn sums_final_arrangement_boxes() {
        let input = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<";

        let map = determine_arrangement_after_moves(input);
        assert_eq!(map.sum_box_coordinates(), 2028);
    }

    #[test]
    fn sums_final_arrangement_boxes_big_example() {
        let input = "##########\n#..O..O.O#\n#......O.#\n#.OO..O.O#\n#..O@..O.#\n#O#..O...#\n#O..O..O.#\n#.OO.O.OO#\n#....O...#\n##########\n\n<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\nvvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\nv^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let map = determine_arrangement_after_moves(input);
        assert_eq!(map.sum_box_coordinates(), 10092);
    }

    #[test]
    fn finds_arrangement_after_5_moves() {
        let input = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>";

        let map = determine_arrangement_after_moves(input);
        assert_eq!(map.get(&(5, 1)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(6, 1)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(4, 2)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(4, 3)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(4, 4)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(4, 5)), Some(&MapFeature::Box));

        // ########
        // #...@OO#
        // ##..O..#
        // #...O..#
        // #.#.O..#
        // #...O..#
        // #......#
        // ########
    }

    #[test]
    fn finds_arrangement_after_7_moves() {
        let input = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>v";

        let map = determine_arrangement_after_moves(input);

        assert_eq!(map.get(&(5, 1)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(6, 1)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(4, 3)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(4, 4)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(4, 5)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(4, 6)), Some(&MapFeature::Box));

        // ########
        // #....OO#
        // ##..@..#
        // #...O..#
        // #.#.O..#
        // #...O..#
        // #...O..#
        // ########
    }

    #[test]
    fn finds_arrangement_after_all_moves() {
        let input = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<";

        let map = determine_arrangement_after_moves(input);
        assert_eq!(map.get(&(5, 1)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(6, 1)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(6, 3)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(3, 4)), Some(&MapFeature::Box));

        assert_eq!(map.get(&(4, 5)), Some(&MapFeature::Box));
        assert_eq!(map.get(&(4, 6)), Some(&MapFeature::Box));

        // ########
        // #....OO#
        // ##.....#
        // #.....O#
        // #.#O@..#
        // #...O..#
        // #...O..#
        // ########
    }
}
