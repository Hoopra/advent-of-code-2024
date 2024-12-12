use crate::util::divide_integer;

use super::{cache::NumberCache, number::Number};

impl Number {
    pub fn from_string(input: &str) -> Self {
        let value = input.parse().unwrap();

        Self { value, blinks: 0 }
    }

    pub fn new(value: u64, blinks: usize) -> Self {
        Self { value, blinks }
    }
}

impl Number {
    fn split(&self, value: u64) -> Vec<Number> {
        let digits = value.to_string().len();
        let blinks = self.blinks + 1;

        if digits % 2 != 0 {
            return vec![Number::new(value * 2024, blinks)];
        }

        let half = divide_integer(digits as usize, 2.0);
        let digits_string = value.to_string();

        let first = take_digits(&digits_string, 0, half);
        let second = take_digits(&digits_string, half, half);

        vec![Number::new(first, blinks), Number::new(second, blinks)]
    }

    pub fn blink(self) -> Vec<Self> {
        let mut result = Vec::new();

        let new_blinks = self.blinks + 1;

        match self.value {
            0 => result.push(Self::new(1, new_blinks)),
            1 => result.push(Self::new(2024, new_blinks)),
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

    pub fn blink_times_with_cache(self, times: usize, cache: &mut NumberCache) -> Vec<Self> {
        let value = self.value;

        match cache.get(value, times) {
            None => {
                let result = self.blink_times(times);
                cache.set(value, times, result.to_vec());

                result
            }
            Some(entries) => entries.to_vec(),
        }
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
    total_count: usize,
}

impl NumberCollection {
    pub fn from_string(input: &str) -> Self {
        let numbers = input
            .split(" ")
            .into_iter()
            .map(|digits| Number::from_string(digits))
            .collect();

        Self {
            numbers,
            total_count: 0,
        }
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

    pub fn blink_times(&mut self, times: usize, steps: f64, cache: &mut NumberCache) -> usize {
        let mut result = 0;

        let step_size = divide_integer(times, steps);
        let mut unresolved = Vec::from(self.numbers.clone());

        while let Some(next) = unresolved.pop() {
            let value = next.value;
            let blinks = next.blinks;

            if blinks == times {
                result += 1;
                continue;
            }

            match cache.get_count(value) {
                None => {}
                Some(count) => {
                    result += *count;
                    continue;
                }
            }

            let new_entries = next.blink_times(step_size);

            if blinks + step_size == times {
                let count = new_entries.len();

                cache.set_count(value, count);

                result += count;

                continue;
            }

            unresolved.extend(new_entries);
        }

        result
    }

    pub fn count(&self) -> usize {
        if self.total_count > 0 {
            return self.total_count;
        }

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
    fn determines_arrangement_after_6_blinks() {
        let input = "125 17";

        let mut collection = NumberCollection::from_string(input);
        let mut cache = NumberCache::new();

        let result = collection.blink_times(6, 1.0, &mut cache);
        assert_eq!(result, 22)
    }

    #[test]
    fn determines_new_numbers_after_5_blinks() {
        let result = Number::new(17, 0).blink_times(5);

        assert_eq!(
            result,
            vec![
                Number::new(4048, 5),
                Number::new(1, 5),
                Number::new(4048, 5),
                Number::new(8096, 5),
                Number::new(28, 5),
                Number::new(67, 5),
                Number::new(60, 5),
                Number::new(32, 5)
            ]
        );
    }
}
