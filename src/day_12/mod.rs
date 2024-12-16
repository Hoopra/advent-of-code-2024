mod area;
mod map_2d;

use crate::util::read_input;
use map_2d::Map2D;

fn calculate_fence_cost(input: &str) -> usize {
    let map = Map2D::from_string(input);

    let areas = map.find_areas();
    areas.iter().map(|area| area.fence_cost()).sum()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_12/input.txt");

    calculate_fence_cost(&input)
}

fn calculate_fence_cost_with_discount(input: &str) -> usize {
    let map = Map2D::from_string(input);

    let areas = map.find_areas();
    areas
        .iter()
        .map(|area| area.fence_cost_with_discount())
        .sum()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_12/input.txt");

    calculate_fence_cost_with_discount(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_fence_cost() {
        let input = "AAAA\nBBCD\nBBCC\nEEEC";

        let result = calculate_fence_cost(input);
        assert_eq!(result, 140);
    }
}
