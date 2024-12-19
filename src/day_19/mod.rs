mod towel;
mod towel_stripe;

use towel::towel_combinations_from_string;

use crate::util::read_input;

fn find_towel_combinations(input: &str) -> usize {
    let (available, combinations) = towel_combinations_from_string(input);

    combinations
        .iter()
        .enumerate()
        .filter(|(index, towel)| {
            println!("index: {} / {}", index, combinations.len());
            towel.is_combination_possible(&available)
        })
        .count()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_19/input.txt");

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
