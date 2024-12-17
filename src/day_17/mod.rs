mod computer;

use crate::util::read_input;
use computer::{Computer, OutputCache};
use std::collections::HashSet;

fn find_computer_output(input: &str) -> String {
    let mut computer = Computer::from_string(input);
    computer.output()
}

pub fn solve_part_1() -> String {
    let input = read_input("src/day_17/input.txt");

    find_computer_output(&input)
}

fn find_lowest_a_register_for_copy(input: &str) -> u64 {
    let computer = Computer::from_string(input);
    // let mut cache: OutputCache = HashSet::new();
    let mut register_a: u64 = 0;

    loop {
        let mut computer = computer.clone();
        computer.set_register_a(register_a as u64);

        // let result = computer.is_output_copy(Some(&mut cache));
        let result = computer.is_output_copy(None);

        if result {
            return register_a;
        }

        register_a += 1;
    }
}

pub fn solve_part_2() -> u64 {
    let input = read_input("src/day_17/input.txt");

    find_lowest_a_register_for_copy(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_lowest_a_register_for_copy() {
        let input = "Register A: 2024\nRegister B: 0\nRegister C: 0\n\nProgram: 0,3,5,4,3,0";

        let result = find_lowest_a_register_for_copy(input);
        assert_eq!(result, 117440);
    }
}
