mod claw_machine;

use crate::util::read_input;
use claw_machine::ClawMachine;

fn find_lowest_token_price(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|text| {
            let machine = ClawMachine::from_string(text);
            machine.lowest_token_price()
        })
        .sum()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_13/input.txt");

    find_lowest_token_price(&input)
}

fn calculate_lowest_token_price(input: &str) -> u128 {
    input
        .split("\n\n")
        .map(|text| {
            let mut machine = ClawMachine::from_string(text);
            machine.correct_error();

            let (a, b) = machine.calculate_best_prize_combination();
            b + 3 * a
        })
        .sum()
}

pub fn solve_part_2() -> u128 {
    let input = read_input("src/day_13/input.txt");

    calculate_lowest_token_price(&input)
}
