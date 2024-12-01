use std::collections::HashMap;

pub struct CompareList {
    lhs: Vec<u32>,
    rhs: Vec<u32>,
}

impl CompareList {
    pub fn from_string(input: String) -> Self {
        let mut lhs = Vec::new();
        let mut rhs = Vec::new();

        let mut pointer: usize = 0;

        input.lines().for_each(|line| {
            let components = line.split(" ");
            components.for_each(|entry| {
                if entry.is_empty() {
                    return;
                }

                let number = entry.parse().unwrap();

                if pointer == 0 {
                    lhs.push(number);
                } else {
                    rhs.push(number);
                }

                pointer = if pointer == 0 { 1 } else { 0 };
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
        sort_list_asc(&mut self.lhs.clone())
            .iter()
            .zip(sort_list_asc(&mut self.rhs.clone()).iter())
            .fold(0, |result, (a, b)| result + (a).abs_diff(*b))
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

fn sort_list_asc(list: &mut [u32]) -> &[u32] {
    list.sort_by(|a, b| a.cmp(b));

    list
}
