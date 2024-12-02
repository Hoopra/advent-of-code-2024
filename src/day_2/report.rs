pub struct Report {
    levels: Vec<i32>,
}

#[derive(Debug)]
enum Sign {
    Negative,
    Positive,
}

impl Report {
    pub fn from_text(input: &str) -> Self {
        let levels = input
            .split(" ")
            .filter_map(|entry| {
                if entry.is_empty() {
                    return None;
                }

                Some(entry.parse().unwrap())
            })
            .collect();

        Self { levels }
    }

    pub fn from_levels(levels: Vec<i32>) -> Self {
        Self { levels }
    }
}

impl Report {
    pub fn is_safe(&self) -> bool {
        let increases: Vec<i32> = self
            .levels
            .iter()
            .skip(1)
            .enumerate()
            .map(|(index, entry)| {
                let previous = self.levels.get(index).unwrap();

                entry - previous
            })
            .collect();

        let first = increases.get(0).unwrap();

        let sign = match first {
            0 => return false,
            _ if first > &0 => Sign::Positive,
            _ => Sign::Negative,
        };

        increases.iter().all(|entry| {
            let abs_diff = entry.abs();

            let correct_sign = match sign {
                Sign::Negative => entry < &0,
                Sign::Positive => entry > &0,
            };

            correct_sign && abs_diff >= 1 && abs_diff <= 3
        })
    }

    pub fn is_safe_with_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        let levels = &self.levels;

        for skip in 0..levels.len() {
            let new_levels = array_skip_index(levels, skip);
            let new_report = Report::from_levels(new_levels);

            if new_report.is_safe() {
                return true;
            }
        }

        false
    }
}

pub fn reports_from_text(text: String) -> Vec<Report> {
    text.lines().map(|line| Report::from_text(line)).collect()
}

fn array_skip_index(array: &Vec<i32>, skip: usize) -> Vec<i32> {
    let mut new_array = Vec::new();

    for (index, level) in array.iter().enumerate() {
        if index != skip {
            new_array.push(*level);
        }
    }

    new_array
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verifies_report_safety() {
        let input = "7 6 4 2 1";
        assert_eq!(Report::from_text(input).is_safe(), true);

        let input = "1 2 7 8 9";
        assert_eq!(Report::from_text(input).is_safe(), false);

        let input = "8 6 4 4 1";
        assert_eq!(Report::from_text(input).is_safe(), false);

        let input = "1 3 6 7 9";
        assert_eq!(Report::from_text(input).is_safe(), true);
    }

    #[test]
    fn verifies_report_safety_with_dampener() {
        let input = "1 2 7 8 9";
        assert_eq!(Report::from_text(input).is_safe(), false);
        assert_eq!(Report::from_text(input).is_safe_with_dampener(), false);

        let input = "1 3 2 4 5";
        assert_eq!(Report::from_text(input).is_safe(), false);
        assert_eq!(Report::from_text(input).is_safe_with_dampener(), true);

        let input = "8 6 4 4 1";
        assert_eq!(Report::from_text(input).is_safe(), false);
        assert_eq!(Report::from_text(input).is_safe_with_dampener(), true);

        let input = "1 3 6 7 9";
        assert_eq!(Report::from_text(input).is_safe(), true);
        assert_eq!(Report::from_text(input).is_safe_with_dampener(), true);
    }

    #[test]
    fn constructs_array_without_index() {
        let array = Vec::from([0, 1, 2, 3]);

        let result = array_skip_index(&array, 2);
        assert_eq!(result, Vec::from([0, 1, 3]));
    }
}
