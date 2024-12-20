mod map_2d;
mod path;

use map_2d::{coordinate_from_string, Map2D};

use crate::util::{read_input, Position};

pub fn find_shortest_path_in_map(
    input: &str,
    size: (isize, isize),
    add_walls: usize,
) -> Option<usize> {
    let map = Map2D::from_string(input, size, add_walls);

    map.best_path().map(|entry| entry.len())
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_18/input.txt");

    find_shortest_path_in_map(&input, (70, 70), 1024).unwrap()
}

pub fn solve_part_2() -> Option<String> {
    let input = read_input("src/day_18/input.txt");
    let base_walls = 1024;

    let mut map = Map2D::from_string(&input, (70, 70), base_walls);
    let mut path: Vec<Position> = map.best_path().unwrap();

    for line in input.lines().skip(base_walls) {
        let position = coordinate_from_string(line);
        map.add_wall_from_string(line);

        if !path.contains(&position) {
            continue;
        }

        let new_path = map.best_path();

        match new_path {
            None => return Some(line.to_string()),
            Some(new_path) => path = new_path,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_best_path_length() {
        let input = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";

        let result = find_shortest_path_in_map(input, (6, 6), 12);
        assert_eq!(result, Some(22));
    }
}
