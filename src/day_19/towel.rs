use std::{cmp::Ordering, fmt::Debug};

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct Towel {
    pub id: Vec<String>,
    pub stripes: Vec<char>,
}

impl Debug for Towel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stripes: Vec<String> = self
            .stripes
            .iter()
            .map(|stripe| stripe.to_string())
            .collect();

        f.write_str(&format!("|{}|", &stripes.join("")))
    }
}

impl PartialOrd for Towel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.stripes.len().cmp(&other.stripes.len()))
    }
}

impl Ord for Towel {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Towel {
    pub fn from_string(input: &str) -> Self {
        Self {
            stripes: input.chars().collect(),
            id: vec![input.to_string()],
        }
    }

    pub fn many_from_string(input: &str, delimiter: &str) -> Vec<Towel> {
        input
            .split(delimiter)
            .map(|segment| Self::from_string(segment))
            .collect()
    }
}

impl Towel {
    pub fn copy(&self, other: Towel) -> Self {
        let mut copy = self.clone();
        copy.stripes.extend(other.stripes);
        copy.id.extend(other.id);

        copy
    }

    pub fn format_id(&self) -> String {
        self.id.clone().join("")
    }
}
