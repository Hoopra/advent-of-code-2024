mod claw_machine;
mod linear_algebra;
mod model;

use crate::util::read_input;
use claw_machine::ClawMachine;

pub fn find_lowest_token_price(input: &str, add: i64) -> i64 {
    input
        .split("\n\n")
        .map(|text| {
            let mut machine = ClawMachine::from_string(text);
            machine.add_prize(add);

            machine.lowest_token_price()
        })
        .sum()
}

pub fn solve_part_1() -> i64 {
    let input = read_input("src/day_13/input.txt");

    let result = find_lowest_token_price(&input, 0);

    assert_eq!(result, 32026);

    result
}

pub fn solve_part_2() -> i64 {
    let input = read_input("src/day_13/input.txt");

    find_lowest_token_price(&input, 10000000000000)
}
