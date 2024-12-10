mod enabled_switch;
mod multiplication;

use crate::util::read_input;
use enabled_switch::EnabledSwitch;
use multiplication::Multiplication;

fn perform_multiplications(input: String) -> i64 {
    input
        .split("mul")
        .into_iter()
        .filter_map(|entry| Multiplication::from_string(&format!("mul{}", entry)))
        .fold(0, |result, m| result + m.result())
}

fn perform_enabled_multiplications(input: String) -> i64 {
    let switch = EnabledSwitch::from_string(&input);
    let multipliers: &Vec<_> = &input.match_indices("mul(").map(|entry| entry.0).collect();

    multipliers.into_iter().fold(0, |result, index| {
        let is_enabled = switch.is_index_enabled(*index);
        let next_index = multipliers
            .clone()
            .into_iter()
            .find(|entry| entry > &index)
            .unwrap_or(input.len() - 1);

        if !is_enabled {
            return result;
        }

        let text = &input[*index..next_index];
        let mul = Multiplication::from_string(text);

        if mul.is_none() {
            return result;
        }

        result + mul.unwrap().result()
    })
}

pub fn solve_part_1() -> i64 {
    let input = read_input("src/day_03/input.txt");

    perform_multiplications(input)
}

pub fn solve_part_2() -> i64 {
    let input = read_input("src/day_03/input.txt");

    perform_enabled_multiplications(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn converts_string_to_multiplications() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let result = perform_multiplications(input.to_string());
        assert_eq!(result, 161);
    }

    #[test]
    fn converts_string_to_enabled_multiplications() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let result = perform_enabled_multiplications(input.to_string());
        assert_eq!(result, 48);
    }
}
