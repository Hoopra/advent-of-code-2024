mod height_map;

use height_map::HeightMap;

use crate::util::read_input;

fn find_trailhead_score(input: &str) -> usize {
    let map = HeightMap::from_string(input);
    let trails = map.find_trailhead_scores();

    trails.values().sum()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_10/input.txt");

    find_trailhead_score(&input)
}

fn find_distinct_trails(input: &str) -> usize {
    let map = HeightMap::from_string(input);
    let trails = map.find_trailhead_trails();

    trails.values().sum()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_10/input.txt");

    find_distinct_trails(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_trailheads() {
        let input =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        let result = find_trailhead_score(input);

        assert_eq!(result, 36)
    }

    #[test]
    fn finds_distinct_trails() {
        let input =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        let result = find_distinct_trails(input);

        assert_eq!(result, 81)
    }
}
