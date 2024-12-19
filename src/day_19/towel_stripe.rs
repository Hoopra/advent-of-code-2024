use std::fmt::Debug;

#[derive(Hash, PartialEq, Clone, Eq, PartialOrd, Ord)]
pub enum TowelStripe {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl TowelStripe {
    pub fn to_string(&self) -> String {
        let character = match self {
            TowelStripe::White => 'w',
            TowelStripe::Blue => 'u',
            TowelStripe::Black => 'b',
            TowelStripe::Red => 'r',
            TowelStripe::Green => 'g',
        };

        format!("{}", character)
    }
}

impl Debug for TowelStripe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string: String = self.to_string();
        f.write_str(&string)
    }
}
