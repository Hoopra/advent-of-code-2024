mod map_2d;
mod robot;

use crate::util::read_input;
use map_2d::Map2D;
use robot::Robot;

fn widen_map_input(input: &str) -> String {
    let mut result = String::new();

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'O' => {
                    result.push('[');
                    result.push(']');
                }
                '@' => {
                    result.push('@');
                    result.push('.');
                }
                _ => {
                    result.push(c);
                    result.push(c);
                }
            }
        }

        result.push('\n');
    }

    result
}

fn determine_arrangement_after_moves(input: &str, widen_map: bool) -> Map2D {
    let mut components = input.split("\n\n");

    let map_input_raw = components.nth(0).unwrap();
    let map_input = match widen_map {
        true => widen_map_input(map_input_raw),
        false => map_input_raw.to_string(),
    };

    let mut map = Map2D::from_string(&map_input);
    let mut robot = Robot::from_moves(map.start, components.nth(0).unwrap());

    robot.move_in_map(&mut map);

    map
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_15/input.txt");

    let map = determine_arrangement_after_moves(&input, false);
    map.sum_box_coordinates()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_15/input.txt");

    let map = determine_arrangement_after_moves(&input, true);

    // 1543338
    map.sum_box_coordinates()
}

#[cfg(test)]
mod tests {
    use map_2d::MapFeature;

    use super::*;

    #[test]
    fn sums_final_arrangement_boxes() {
        let input = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<";

        let map = determine_arrangement_after_moves(input, false);
        assert_eq!(map.sum_box_coordinates(), 2028);
    }

    #[test]
    fn sums_final_arrangement_boxes_big_example() {
        let input = "##########\n#..O..O.O#\n#......O.#\n#.OO..O.O#\n#..O@..O.#\n#O#..O...#\n#O..O..O.#\n#.OO.O.OO#\n#....O...#\n##########\n\n<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\nvvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\nv^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let map = determine_arrangement_after_moves(input, false);
        assert_eq!(map.sum_box_coordinates(), 10092);
    }

    #[test]
    fn finds_arrangement_after_5_moves() {
        let input = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>";

        let map = determine_arrangement_after_moves(input, false);
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

        let map = determine_arrangement_after_moves(input, false);

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

        let map = determine_arrangement_after_moves(input, false);
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

    #[test]
    fn finds_arrangement_after_all_moves_wide_map() {
        let input = "#######\n#...#.#\n#.....#\n#..OO@#\n#..O..#\n#.....#\n#######\n\n<vv<<^^<<^^";

        let map = determine_arrangement_after_moves(input, true);

        assert_eq!(map.get(&(5, 1)), Some(&MapFeature::BoxLeft));
        assert_eq!(map.get(&(6, 1)), Some(&MapFeature::BoxRight));

        assert_eq!(map.get(&(7, 2)), Some(&MapFeature::BoxLeft));
        assert_eq!(map.get(&(8, 2)), Some(&MapFeature::BoxRight));

        assert_eq!(map.get(&(6, 3)), Some(&MapFeature::BoxLeft));
        assert_eq!(map.get(&(7, 3)), Some(&MapFeature::BoxRight));

        assert_eq!(map.sum_box_coordinates(), 618);

        // ##############
        // ##...[].##..##
        // ##...@.[]...##
        // ##....[]....##
        // ##..........##
        // ##..........##
        // ##############
    }

    #[test]
    fn finds_arrangement_after_all_moves_large_wide_map() {
        let input = "##########\n#..O..O.O#\n#......O.#\n#.OO..O.O#\n#..O@..O.#\n#O#..O...#\n#O..O..O.#\n#.OO.O.OO#\n#....O...#\n##########\n\n<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\nvvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\nv^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        let map = determine_arrangement_after_moves(input, true);

        assert_eq!(map.sum_box_coordinates(), 9021);

        // ####################
        // ##[].......[].[][]##
        // ##[]...........[].##
        // ##[]........[][][]##
        // ##[]......[]....[]##
        // ##..##......[]....##
        // ##..[]............##
        // ##..@......[].[][]##
        // ##......[][]..[]..##
        // ####################
    }

    #[test]
    fn moves_robot_left() {
        let input = "###########\n#..OO.OO@.#\n###########\n\n<<<";

        let map = determine_arrangement_after_moves(input, true);

        assert_eq!(map.sum_box_coordinates(), 432);
    }

    #[test]
    fn moves_robot_right() {
        let input = "###########\n#..@OO.OO.#\n###########\n\n>>>";

        let map = determine_arrangement_after_moves(input, true);

        assert_eq!(map.sum_box_coordinates(), 452);
    }

    #[test]
    fn moves_robot_up() {
        let input =
            "########\n#......#\n#.[][].#\n#......#\n#..[]..#\n#.[]...#\n#..@...#\n########\n\n^^";

        let map = determine_arrangement_after_moves(input, false);

        assert_eq!(map.sum_box_coordinates(), 711);
    }

    #[test]
    fn moves_robot_down() {
        let input =
            "########\n#..@...#\n#..[]..#\n#.[][].#\n#......#\n#[]..[]#\n#......#\n########\n\nvv";

        let map = determine_arrangement_after_moves(input, false);

        assert_eq!(map.sum_box_coordinates(), 2615);
    }
}
