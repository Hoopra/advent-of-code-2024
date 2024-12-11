use crate::util::divide_integer;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Stone {
    number: u64,
}

impl Stone {
    pub fn from_string(input: &str) -> Self {
        Self {
            number: input.parse().unwrap(),
        }
    }

    pub fn new(number: u64) -> Self {
        Self { number }
    }
}

impl Stone {
    fn attempt_split(self) -> Vec<Self> {
        let digits_string = self.number.to_string();
        let length = digits_string.len();

        if length % 2 == 1 {
            return vec![Self::new(self.number * 2024)];
        }

        let each_length = divide_integer(length, 2.0);
        let first = take_digits(&digits_string, 0, each_length);
        let second = take_digits(&digits_string, each_length, each_length);

        vec![Self::new(first), Self::new(second)]
    }

    pub fn blink(self) -> Vec<Self> {
        let mut result = Vec::new();

        match self.number {
            0 => result.push(Self::new(1)),
            _ => result.extend(self.attempt_split()),
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
    stones: Vec<Stone>,
}

impl StoneCollection {
    pub fn from_string(input: &str) -> Self {
        let stones = input
            .split(" ")
            .into_iter()
            .map(|digits| Stone::from_string(digits))
            .collect();

        Self { stones }
    }

    pub fn new(stones: Vec<Stone>) -> Self {
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
                Stone { number: 1 },
                Stone { number: 2024 },
                Stone { number: 1 },
                Stone { number: 0 },
                Stone { number: 9 },
                Stone { number: 9 },
                Stone { number: 2021976 }
            ]
        );
    }
}
