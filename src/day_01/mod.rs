mod compare_list;

use crate::util::read_input;
use compare_list::CompareList;

fn list_difference_from_input(input: String) -> u32 {
    let compare_list = CompareList::from_string(input);

    compare_list.find_difference()
}

pub fn solve_part_1() -> u32 {
    let input = read_input("src/day_1/input.txt");

    list_difference_from_input(input)
}

fn list_similarity_from_input(input: String) -> u32 {
    let compare_list = CompareList::from_string(input);

    compare_list.find_similarity()
}

pub fn solve_part_2() -> u32 {
    let input = read_input("src/day_1/input.txt");

    list_similarity_from_input(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_difference() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

        let result = list_difference_from_input(input.to_string());
        assert_eq!(result, 11);
    }

    #[test]
    fn finds_similarity() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

        let result = list_similarity_from_input(input.to_string());
        assert_eq!(result, 31);
    }
}
