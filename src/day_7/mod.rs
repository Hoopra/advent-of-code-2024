mod test_equation;

use crate::util::read_input;
use test_equation::{Operator, TestEquation};

fn part_1_operations() -> Vec<Operator> {
    Vec::from(Vec::from([Operator::Add, Operator::Multiply]))
}

fn part_2_operations() -> Vec<Operator> {
    Vec::from(Vec::from([
        Operator::Add,
        Operator::Multiply,
        Operator::Concatenate,
    ]))
}

fn sum_possible_equations(input: &str, available_operators: &Vec<Operator>) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let equation = TestEquation::from_string(line);

            match equation.is_valid_with_operators(None, available_operators) {
                true => Some(equation.result),
                false => None,
            }
        })
        .sum()
}

pub fn solve_part_1() -> u64 {
    let input = read_input("src/day_7/input.txt");

    let available = part_1_operations();
    sum_possible_equations(&input, &available)
}

pub fn solve_part_2() -> u64 {
    let input = read_input("src/day_7/input.txt");

    // TODO: optimise - very slow
    let available = part_2_operations();
    sum_possible_equations(&input, &available)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_possible_equations() {
        let input = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";

        let available = part_1_operations();
        let result = sum_possible_equations(input, &available);

        assert_eq!(result, 3749);
    }

    #[test]
    fn sums_possible_equations_with_concatenation() {
        let input = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";

        let available = part_2_operations();
        let result = sum_possible_equations(input, &available);

        assert_eq!(result, 11387);
    }
}
