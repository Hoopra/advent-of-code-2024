mod map_2d;

use crate::util::read_input;
use map_2d::Map2D;

fn count_antinodes(input: &str) -> usize {
    let map = Map2D::from_string(input);

    map.find_antinodes(false).len()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_08/input.txt");

    count_antinodes(&input)
}

fn count_antinodes_with_resonance(input: &str) -> usize {
    let map = Map2D::from_string(input);

    map.find_antinodes(true).len()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_08/input.txt");

    count_antinodes_with_resonance(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_antinodes_in_map() {
        let input = "..........\n..........\n..........\n....a.....\n........a.\n.....a....\n..........\n..........\n..........\n..........";

        let result = count_antinodes(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn counts_antinodes_in_complex_map() {
        let input = ".............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............";

        let result = count_antinodes(input);
        assert_eq!(result, 14);
    }

    #[test]
    fn counts_antinodes_with_resonance() {
        let input = "T.........\n...T......\n.T........\n..........\n..........\n..........\n..........\n..........\n..........\n..........";

        let result = count_antinodes_with_resonance(input);
        assert_eq!(result, 9);
    }
}
