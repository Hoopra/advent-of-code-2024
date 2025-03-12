use super::{
    linear_algebra::solve_2n_order_system,
    model::{text_to_button, text_to_prize, Coordinate},
};

pub struct ClawMachine {
    button_a: Coordinate,
    button_b: Coordinate,
    prize: Coordinate,
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

fn combination_price(a: &Coordinate) -> i64 {
    a.0 * 3 + a.1 * 1
}

impl ClawMachine {
    pub fn add_prize(&mut self, add: i64) {
        self.prize = (self.prize.0 + add, self.prize.1 + add);
    }

    pub fn lowest_token_price(&self) -> i64 {
        let (a1, a2) = self.button_a;
        let (b1, b2) = self.button_b;

        let matrix = [(a1, b1), (a2, b2)];
        let result = solve_2n_order_system(&matrix, &self.prize);

        combination_price(&result)
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

        let input = "Button A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176";
        let machine = ClawMachine::from_string(input);

        assert_eq!(machine.lowest_token_price(), 0);

        let input = "Button A: X+69, Y+23\nButton B: X+27, Y+71\nPrize: X=18641, Y=10279";
        let machine = ClawMachine::from_string(input);

        assert_eq!(machine.lowest_token_price(), 0);
    }
}
