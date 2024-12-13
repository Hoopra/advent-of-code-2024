use super::number::Number;
use crate::util::divide_integer;

impl Number {
    pub fn from_string(input: &str) -> Self {
        let value = input.parse().unwrap();

        Self { value }
    }

    pub fn new(value: u64) -> Self {
        Self { value }
    }
}

impl Number {
    fn split(&self, value: u64) -> Vec<Number> {
        let digits = value.to_string().len();

        if digits % 2 != 0 {
            return vec![Number::new(value * 2024)];
        }

        let half = divide_integer(digits as usize, 2.0);
        let digits_string = value.to_string();

        let first = take_digits(&digits_string, 0, half);
        let second = take_digits(&digits_string, half, half);

        vec![Number::new(first), Number::new(second)]
    }

    pub fn blink(self) -> Vec<Self> {
        let mut result = Vec::new();

        match self.value {
            0 => result.push(Self::new(1)),
            1 => result.push(Self::new(2024)),
            value => result.extend(self.split(value)),
        }

        result
    }
}
fn take_digits(digits: &str, skip: usize, take: usize) -> u64 {
    digits
        .chars()
        .skip(skip)
        .take(take)
        .collect::<String>()
        .parse()
        .unwrap()
}

pub struct NumberCollection {
    numbers: Vec<Number>,
}

impl NumberCollection {
    pub fn from_string(input: &str) -> Self {
        let numbers = input
            .split(" ")
            .into_iter()
            .map(|digits| Number::from_string(digits))
            .collect();

        Self { numbers }
    }
}

impl NumberCollection {
    pub fn blink(&mut self) {
        self.numbers = self
            .numbers
            .clone()
            .into_iter()
            .flat_map(|number| number.blink())
            .collect()
    }

    pub fn count(&self) -> usize {
        self.numbers.len()
    }

    #[cfg(test)]
    pub fn to_numbers(&self) -> Vec<u64> {
        self.numbers.iter().map(|number| number.value).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_arrangement_after_1_blink() {
        let input = "0 1 10 99 999";

        let mut collection = NumberCollection::from_string(input);
        collection.blink();

        assert_eq!(collection.to_numbers(), vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }
}
