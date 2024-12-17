mod computer;

use crate::util::read_input;
use computer::Computer;

pub fn find_computer_output(input: &str) -> String {
    let mut computer = Computer::from_string(input);
    computer.output()
}

pub fn solve_part_1() -> String {
    let input = read_input("src/day_17/input.txt");
    // let input = "Register A: 729\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1,5,4,3,0".to_string();

    find_computer_output(&input)
}

pub fn solve_part_2() -> u128 {
    let input = read_input("src/day_13/input.txt");

    0
}
