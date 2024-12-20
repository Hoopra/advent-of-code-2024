mod combine;
mod towel;

use crate::util::read_input;
use combine::towel_combinations_from_string;

fn find_towel_combinations(input: &str) -> usize {
    let (available, combinations) = towel_combinations_from_string(input);

    combinations
        .iter()
        .filter(|towel| towel.is_combination_possible(&available))
        .count()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_19/input.txt");

    // TODO: Optimise
    find_towel_combinations(&input)
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_19/input.txt");

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_towel_combinations() {
        let input =
            "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";

        let result = find_towel_combinations(input);
        assert_eq!(result, 6);
    }
}
