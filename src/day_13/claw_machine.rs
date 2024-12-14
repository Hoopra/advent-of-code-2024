use std::cmp::Ordering;

type Coordinate = (usize, usize);

pub struct ClawMachine {
    button_a: Coordinate,
    button_b: Coordinate,
    prize: Coordinate,
}

fn text_to_button(input: &str) -> Coordinate {
    let next: String = input.chars().skip(10).collect();
    let entries: Vec<&str> = next.split(", ").collect();

    (
        text_to_coordinate(&entries.get(0).unwrap()),
        text_to_coordinate(&entries.get(1).unwrap()),
    )
}

fn text_to_coordinate(input: &str) -> usize {
    input.chars().skip(2).collect::<String>().parse().unwrap()
}

fn text_to_prize(input: &str) -> Coordinate {
    let next: String = input.chars().skip(7).collect();
    let entries: Vec<&str> = next.split(", ").collect();

    (
        text_to_coordinate(&entries.get(0).unwrap()),
        text_to_coordinate(&entries.get(1).unwrap()),
    )
}

impl ClawMachine {
    pub fn from_string(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        Self {
            button_a: text_to_button(lines.get(0).unwrap()),
            button_b: text_to_button(lines.get(1).unwrap()),
            prize: text_to_prize(lines.get(2).unwrap()),
        }
    }
}

fn is_valid_combination(a: usize, times_a: usize, b: usize, times_b: usize, target: usize) -> bool {
    if times_a > 100 || times_b > 100 {
        return false;
    }

    a * times_a + b * times_b == target
}

fn combinations(a: usize, b: usize, p: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for times_a in 0..100 {
        for times_b in 0..100 {
            let combination = (times_a, times_b);
            if is_valid_combination(a, combination.0, b, combination.1, p) {
                result.push(combination);
            }
        }
    }

    result
}

fn combination_price(a: &(usize, usize)) -> usize {
    a.0 * 3 + a.1 * 1
}

fn compare_button_presses(a: &(usize, usize), b: &(usize, usize)) -> Ordering {
    combination_price(a).cmp(&combination_price(b))
}

impl ClawMachine {
    fn combinations_x(&self) -> Vec<(usize, usize)> {
        combinations(self.button_a.0, self.button_b.0, self.prize.0)
    }

    fn combinations_y(&self) -> Vec<(usize, usize)> {
        combinations(self.button_a.1, self.button_b.1, self.prize.1)
    }

    pub fn lowest_token_price(&self) -> usize {
        let combinations_x = self.combinations_x();
        let combinations_y = self.combinations_y();

        let combination = combinations_x
            .iter()
            .filter(|combination| combinations_y.contains(combination))
            .min_by(|a, b| compare_button_presses(a, b));

        match combination {
            None => 0,
            Some(combination) => combination_price(combination),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ClawMachine;

    #[test]
    fn determines_lowest_token_price() {
        let input = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400";
        let machine = ClawMachine::from_string(input);

        assert_eq!(machine.button_a, (94, 34));
        assert_eq!(machine.button_b, (22, 67));
        assert_eq!(machine.prize, (8400, 5400));
        assert_eq!(machine.lowest_token_price(), 280);
    }
}
