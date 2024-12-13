use super::number::Number;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct NumberMap {
    numbers: HashMap<u64, usize>,
}

impl NumberMap {
    pub fn from_string(input: &str) -> Self {
        let mut numbers = HashMap::new();

        for value_string in input.split(" ") {
            let value: u64 = value_string.parse().unwrap();

            match numbers.get(&value) {
                Some(count) => numbers.insert(value, count + 1),
                None => numbers.insert(value, 1),
            };
        }

        Self { numbers }
    }
}

impl NumberMap {
    fn add(&mut self, value: u64, count: usize) {
        let current = self.numbers.get(&value).unwrap_or(&0);

        self.numbers.insert(value, current + count);
    }

    fn remove(&mut self, value: u64, count: usize) {
        let found = self.numbers.get(&value).unwrap_or(&0);

        match found {
            current if current <= &count => self.numbers.remove(&value),
            current => self.numbers.insert(value, current - count),
        };
    }

    pub fn blink(&mut self) {
        for (value, times) in self.numbers.clone().iter() {
            if times == &0 {
                self.numbers.remove(value);
                continue;
            }

            self.remove(*value, *times);

            let number = Number::new(*value);
            let numbers = number.blink();

            for number in numbers {
                let value = number.value;
                self.add(value, *times);
            }
        }
    }

    pub fn blink_times(&mut self, times: usize) {
        for _ in 0..times {
            self.blink();
        }
    }

    pub fn count(&self) -> usize {
        self.numbers.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_arrangement_after_1_blink() {
        let input = "125 17";

        let mut collection = NumberMap::from_string(input);
        collection.blink_times(1);

        assert_eq!(
            collection.numbers,
            HashMap::from([(253000, 1), (1, 1), (7, 1),])
        );
    }

    #[test]
    fn determines_arrangement_after_2_blinks() {
        let input = "125 17";

        let mut collection = NumberMap::from_string(input);
        collection.blink_times(2);

        assert_eq!(
            collection.numbers,
            HashMap::from([(253, 1), (0, 1), (2024, 1), (14168, 1)])
        );
    }

    #[test]
    fn determines_arrangement_after_3_blinks() {
        let input = "125 17";

        let mut collection = NumberMap::from_string(input);
        collection.blink_times(3);

        assert_eq!(
            collection.numbers,
            HashMap::from([(512072, 1), (1, 1), (20, 1), (24, 1), (28676032, 1)])
        );
    }

    #[test]
    fn determines_arrangement_after_4_blinks() {
        let input = "125 17";

        let mut collection = NumberMap::from_string(input);
        collection.blink_times(4);

        assert_eq!(
            collection.numbers,
            HashMap::from([
                (512, 1),
                (72, 1),
                (2024, 1),
                (2, 2),
                (0, 1),
                (4, 1),
                (2867, 1),
                (6032, 1)
            ])
        );
    }

    #[test]
    fn determines_arrangement_after_5_blinks() {
        let input = "125 17";

        let mut collection = NumberMap::from_string(input);
        collection.blink_times(5);

        assert_eq!(
            collection.numbers,
            HashMap::from([
                (1036288, 1),
                (7, 1),
                (2, 1),
                (20, 1),
                (24, 1),
                (1, 1),
                (4048, 2),
                (8096, 1),
                (28, 1),
                (67, 1),
                (60, 1),
                (32, 1),
            ])
        );
    }

    #[test]
    fn determines_arrangement_after_6_blinks() {
        let input = "125 17";

        let mut collection = NumberMap::from_string(input);
        collection.blink_times(6);

        assert_eq!(
            collection.numbers,
            HashMap::from([
                (2097446912, 1),
                (14168, 1),
                (4048, 1),
                (2, 4),
                (0, 2),
                (4, 1),
                (40, 2),
                (48, 2),
                (2024, 1),
                (80, 1),
                (96, 1),
                (8, 1),
                (6, 2),
                (7, 1),
                (3, 1),
            ])
        );

        // left: {2: 1, 7: 0 }
        // right: {7: 1, 2: 4 }
    }

    #[test]
    fn determines_total_after_25_blinks() {
        let input = "125 17";

        let mut collection = NumberMap::from_string(input);
        collection.blink_times(25);

        assert_eq!(collection.count(), 55312);
    }
}
