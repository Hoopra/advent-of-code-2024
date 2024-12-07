#[derive(Clone, Copy)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

pub struct TestEquation {
    pub result: u64,
    entries: Vec<u64>,
}

impl TestEquation {
    pub fn from_string(input: &str) -> Self {
        let components: Vec<&str> = input.split(":").collect();

        let result = components.iter().nth(0).unwrap().parse().unwrap();
        let entries = components
            .iter()
            .nth(1)
            .unwrap()
            .split(" ")
            .filter_map(|entry| match entry.is_empty() {
                true => None,
                false => Some(entry.parse().unwrap()),
            })
            .collect();

        Self { result, entries }
    }
}

impl TestEquation {
    fn perform_operations(&self, operators: &Vec<Operator>) -> Option<u64> {
        let slots = self.entries.len() - 1;

        if operators.len() != slots {
            return None;
        }

        let result =
            self.entries
                .iter()
                .enumerate()
                .fold(0, |result, (index, value)| match index {
                    0 => *value,
                    _ => {
                        let operator = operators.get(index - 1).unwrap();
                        match operator {
                            Operator::Add => result + value,
                            Operator::Multiply => result * value,
                            Operator::Concatenate => {
                                format!("{result}{value}").parse::<u64>().unwrap()
                            }
                        }
                    }
                });

        Some(result)
    }

    fn is_valid_with_extra_operators(
        &self,
        operators: Vec<Operator>,
        available: &Vec<Operator>,
    ) -> bool {
        available.iter().any(|operator| {
            let mut new_operators = Vec::from(&operators[..]);
            new_operators.push(*operator);

            self.is_valid_with_operators(Some(new_operators), available)
        })
    }

    pub fn is_valid_with_operators(
        &self,
        operators: Option<Vec<Operator>>,
        available: &Vec<Operator>,
    ) -> bool {
        match operators {
            None => self.is_valid_with_extra_operators(Vec::new(), available),
            Some(operators) => {
                let result = self.perform_operations(&operators);

                match result {
                    None => self.is_valid_with_extra_operators(operators, available),
                    Some(result) => result == self.result,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_equation_validity() {
        let input = "190: 10 19";

        let equation = TestEquation::from_string(input);
        assert_eq!(equation.result, 190);
        assert_eq!(equation.entries, Vec::from([10, 19]));

        let available = Vec::from([Operator::Add, Operator::Multiply]);
        let result = equation.is_valid_with_operators(None, &available);
        assert_eq!(result, true);
    }
}
