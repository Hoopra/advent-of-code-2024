pub struct EnabledSwitch {
    ranges: Vec<(usize, bool)>,
}

impl EnabledSwitch {
    pub fn from_string(input: &str) -> Self {
        let enablers: &Vec<_> = &input.match_indices("do()").collect();
        let disablers: &Vec<_> = &input.match_indices("don't()").collect();

        let mut commands: Vec<(usize, &str)> = disablers
            .into_iter()
            .chain(enablers)
            .map(|entry| *entry)
            .collect();
        commands.sort();

        let mut ranges: Vec<(usize, bool)> = Vec::from([(0, true)]);

        commands.iter().enumerate().for_each(|(_, next)| {
            let previous = ranges.get(ranges.len() - 1).unwrap();
            let current_state = previous.1;
            let new_state = if next.1 == "do()" { true } else { false };

            if new_state != current_state {
                ranges.push((next.0, new_state));
            }
        });

        Self { ranges }
    }

    #[cfg(test)]
    pub fn from_ranges(ranges: &[(usize, bool)]) -> Self {
        Self {
            ranges: Vec::from(ranges),
        }
    }
}

impl EnabledSwitch {
    pub fn is_index_enabled(&self, i: usize) -> bool {
        match &self.ranges.iter().rfind(|(position, _)| position < &i) {
            None => return true,
            Some((_, enabled)) => *enabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_identifies_enabled_state() {
        let ranges = [(0, true), (20, false), (59, true)];
        let switch = EnabledSwitch::from_ranges(&ranges);

        assert_eq!(switch.is_index_enabled(0), true);
        assert_eq!(switch.is_index_enabled(12), true);
        assert_eq!(switch.is_index_enabled(21), false);
        assert_eq!(switch.is_index_enabled(40), false);
        assert_eq!(switch.is_index_enabled(64), true);
    }
}
