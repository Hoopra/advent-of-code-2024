mod claw_machine;

use crate::util::read_input;
use claw_machine::ClawMachine;

pub fn find_lowest_token_price(input: &str) -> usize {
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

pub fn solve_part_2() -> usize {
    let _input = read_input("src/day_13/input.txt");

    0
}
