mod computer;

use crate::util::read_input;
use computer::Computer;

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
    let mut register_a: u64 = 0;

    loop {
        let mut computer = computer.clone();
        computer.set_register_a(register_a as u64);

        let (result, difference) = computer.is_output_copy();
        println!("register_a: {}, difference {}", register_a, difference);

        if result {
            return register_a;
        }

        if difference < 0 {
            register_a -= 1;
            continue;
        }

        register_a += match difference as u64 {
            0 => 1,
            value => value * 100000000,
        };

        // register_a += match difference.abs() {
        //     value if value >= 1 => value * 100000,
        //     value if value >= 5 => value * 100000000000,
        //     _ => 1,
        // };
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
