use crate::util::divide_integer;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Number {
    Zero,
    One,
    Value(u64, usize),
}

impl Number {
    pub fn from_string(input: &str) -> Self {
        match input.parse().unwrap() {
            0 => Number::Zero,
            1 => Number::One,
            value => Number::Value(value, input.len()),
        }
    }

    pub fn from_value(value: u64, digits: Option<usize>) -> Self {
        match value {
            0 => Self::Zero,
            1 => Self::One,
            _ => {
                let digits = digits.unwrap_or(value.to_string().len());
                Self::Value(value, digits)
            }
        }
    }
}

fn split_number_by_digits(value: u64, digits: usize) -> Vec<Number> {
    if digits % 2 == 0 {
        let half = divide_integer(digits as usize, 2.0);
        let digits_string = value.to_string();

        let first = take_digits(&digits_string, 0, half);
        let second = take_digits(&digits_string, half, half);

        return vec![
            Number::from_value(first, None),
            Number::from_value(second, None),
        ];
    }

    // let multiplier = 4;
    // let result = ((digits as f64).log10() + (4.0_f64).log10()).floor() + 1.0;

    vec![Number::from_value(value * 2024, None)]
}

impl Number {
    pub fn blink(self) -> Vec<Self> {
        let mut result = Vec::new();

        match self {
            Number::Zero => result.push(Number::One),
            Number::One => result.push(Number::Value(2024, 4)),
            Number::Value(value, digits) => result.extend(split_number_by_digits(value, digits)),
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

pub struct StoneCollection {
    stones: Vec<Number>,
}

impl StoneCollection {
    pub fn from_string(input: &str) -> Self {
        let stones = input
            .split(" ")
            .into_iter()
            .map(|digits| Number::from_string(digits))
            .collect();

        Self { stones }
    }

    fn new(stones: Vec<Number>) -> Self {
        Self { stones }
    }
}

impl StoneCollection {
    pub fn blink(&mut self) {
        self.stones = self
            .stones
            .clone()
            .into_iter()
            .flat_map(|stone| stone.blink())
            .collect()
    }

    pub fn split(self) -> Vec<Self> {
        let length = self.count();
        let each_length = divide_integer(length, 2.0);

        let mut first = vec![];
        let mut second = vec![];

        for (i, stone) in self.stones.iter().enumerate() {
            if i < each_length {
                first.push(stone.clone());
            } else {
                second.push(stone.clone());
            }
        }

        vec![Self::new(first), Self::new(second)]
    }

    pub fn count(&self) -> usize {
        self.stones.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_arrangement_after_1_blink() {
        let input = "0 1 10 99 999";

        let mut collection = StoneCollection::from_string(input);
        collection.blink();

        assert_eq!(
            collection.stones,
            vec![
                Number::One,
                Number::Value(2024, 4),
                Number::One,
                Number::Zero,
                Number::Value(9, 1),
                Number::Value(9, 1),
                Number::Value(2021976, 7)
            ]
        );
    }
}
