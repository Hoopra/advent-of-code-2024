use super::number::Number;
use crate::util::divide_integer;
use std::collections::HashMap;

type NumberCache = HashMap<u64, usize>;

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

    fn blink_times(self, times: usize) -> Vec<Self> {
        let mut result = vec![self];

        for _ in 0..times {
            let new_entries = result.clone();

            result.clear();

            result.extend(
                new_entries
                    .into_iter()
                    .flat_map(|entry: Number| entry.blink()),
            );
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

    pub fn with_numbers(numbers: Vec<Number>) -> Self {
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

    pub fn blink_times_and_count(
        self,
        step_size: usize,
        current_times: usize,
        total: usize,
        cache: &mut NumberCache,
    ) -> usize {
        self.numbers
            .into_iter()
            .map(|number| {
                let value = number.value;
                let has_cached_value = cache.contains_key(&value);

                if current_times == total && has_cached_value {
                    return *cache.get(&value).unwrap();
                }

                let numbers = number.blink_times(step_size);

                if !has_cached_value {
                    cache.insert(value, numbers.len());
                }

                match current_times == total {
                    false => {
                        let collection = NumberCollection::with_numbers(numbers);
                        collection.blink_times_and_count(
                            step_size,
                            current_times + step_size,
                            total,
                            cache,
                        )
                    }
                    true => numbers.len(),
                }
            })
            .sum()
    }

    pub fn blink_times(self, times: usize, step_size: usize) -> usize {
        let mut cache = HashMap::new();

        self.blink_times_and_count(step_size, step_size, times, &mut cache)
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

    #[test]
    fn determines_new_numbers_after_5_blinks() {
        let result = Number::new(17).blink_times(5);

        assert_eq!(
            result,
            vec![
                Number::new(4048),
                Number::new(1),
                Number::new(4048),
                Number::new(8096),
                Number::new(28),
                Number::new(67),
                Number::new(60),
                Number::new(32)
            ]
        );
    }

    #[test]
    fn determines_arrangement_after_25_blinks_with_cache() {
        let input = "125 17";

        let collection = NumberCollection::from_string(input);
        let result = collection.blink_times(25, 5);

        assert_eq!(result, 55312);
    }
}
