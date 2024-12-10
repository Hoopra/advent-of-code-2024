mod height_map;

use height_map::HeightMap;

use crate::util::read_input;

fn find_trailheads(input: &str) -> usize {
    let map = HeightMap::from_string(input);
    let trails = map.find_trails();

    trails.values().sum()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_9/input.txt");

    find_trailheads(&input)
}

pub fn solve_part_2() -> usize {
    let _input = read_input("src/day_9/input.txt");

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_trailheads() {
        let input =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        let result = find_trailheads(input);

        assert_eq!(result, 36)
    }
}
