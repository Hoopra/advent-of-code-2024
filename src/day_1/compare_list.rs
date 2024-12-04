use std::{cmp::Ordering, collections::HashMap};

pub struct CompareList {
    lhs: Vec<u32>,
    rhs: Vec<u32>,
}

impl CompareList {
    pub fn from_string(input: String) -> Self {
        let mut lhs = Vec::new();
        let mut rhs = Vec::new();

        input.lines().for_each(|line| {
            line.split(" ").for_each(|entry| {
                if entry.is_empty() {
                    return;
                }

                let number = entry.parse().unwrap();

                match lhs.len() == rhs.len() {
                    true => lhs.push(number),
                    false => rhs.push(number),
                }
            });
        });

        Self {
            lhs: lhs.to_vec(),
            rhs: rhs.to_vec(),
        }
    }
}

impl CompareList {
    pub fn find_difference(&self) -> u32 {
        let mut lhs_sorted = self.lhs.clone();
        let mut rhs_sorted = self.rhs.clone();

        lhs_sorted.sort_by(sort_asc);
        rhs_sorted.sort_by(sort_asc);

        lhs_sorted
            .iter()
            .zip(rhs_sorted)
            .fold(0, |result, (a, b)| result + (a).abs_diff(b))
    }

    pub fn find_similarity(&self) -> u32 {
        let rhs_occurrence = self.rhs.iter().fold(HashMap::new(), |mut result, entry| {
            let times: &u32 = result.get(entry).unwrap_or(&0);

            result.insert(entry, times + 1);

            result
        });

        self.lhs.iter().fold(0, |sum, entry| {
            let occurrence = rhs_occurrence.get(entry).unwrap_or(&0);
            sum + entry * occurrence
        })
    }
}

fn sort_asc(a: &u32, b: &u32) -> Ordering {
    a.cmp(&b)
}
