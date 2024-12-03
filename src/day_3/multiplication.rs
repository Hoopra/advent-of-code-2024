#[derive(Debug, PartialEq)]
pub struct Multiplication {
    lhs: i64,
    rhs: i64,
}

impl Multiplication {
    pub fn from_string(input: &str) -> Option<Self> {
        let mut lhs: Vec<char> = Vec::new();
        let mut rhs: Vec<char> = Vec::new();

        let mut pointer: usize = 0;

        if !input.starts_with("mul") {
            return None;
        }

        for char in input.chars().skip(3) {
            match char {
                '(' => {}
                ',' => pointer = 1,
                ')' => {
                    break;
                }
                c if c.is_numeric() => match pointer {
                    0 => lhs.push(c),
                    _ => rhs.push(c),
                },
                _ => return None,
            }
        }

        Some(Self {
            lhs: lhs.iter().collect::<String>().parse().unwrap(),
            rhs: rhs.iter().collect::<String>().parse().unwrap(),
        })
    }
}

impl Multiplication {
    pub fn result(&self) -> i64 {
        self.lhs * self.rhs
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn converts_valid_string_to_multiplication() {
        let input = "mul(12,32)";

        let multiplication = Multiplication::from_string(input);
        assert_ne!(multiplication, None);
    }

    #[test]
    fn converts_invalid_string_to_none() {
        let input = "mul[12,32)";

        let multiplication = Multiplication::from_string(input);
        assert_eq!(multiplication, None);
    }
}
