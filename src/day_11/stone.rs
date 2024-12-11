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
}

impl Stone {
    fn split(self) -> Vec<Self> {
        let digits = self.number.to_string();

        let length = digits.len();
        let each_length = ((length as f64) / 2.0).floor() as usize;

        let components = digits.chars();
        let first = &components
            .clone()
            .take(each_length)
            .collect::<String>()
            .parse()
            .unwrap();

        let last: u64 = components
            .skip(each_length)
            .take(each_length)
            .collect::<String>()
            .parse()
            .unwrap();

        vec![Self { number: *first }, Self { number: last }]
    }

    pub fn blink(self) -> Vec<Self> {
        let mut result = Vec::new();

        match self.number {
            0 => {
                result.push(Self { number: 1 });
            }
            number if number.to_string().len() % 2 == 0 => result.extend(self.split()),
            number => result.push(Self {
                number: number * 2024,
            }),
        }

        result
    }
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
