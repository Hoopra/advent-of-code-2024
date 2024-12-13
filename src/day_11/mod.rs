mod collection;
mod number;

use crate::util::read_input;
use collection::NumberCollection;

pub fn arrangement_after_blinks(input: &str, blinks: u64) -> usize {
    let mut collection = NumberCollection::from_string(input);

    for _ in 0..blinks {
        collection.blink();
    }

    collection.count()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_11/input.txt");

    arrangement_after_blinks(&input, 25)
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_11/input.txt");

    let collection = NumberCollection::from_string(&input);

    collection.blink_times(75, 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_1_with_step_size_5() {
        let input = read_input("src/day_11/input.txt");

        let collection = NumberCollection::from_string(&input);

        let result = collection.blink_times(25, 5);
        assert_eq!(result, 183620);
    }

    #[test]
    fn solves_part_1_with_step_size_1() {
        let input = read_input("src/day_11/input.txt");

        let collection = NumberCollection::from_string(&input);

        let result = collection.blink_times(25, 1);
        assert_eq!(result, 183620);
    }
}
