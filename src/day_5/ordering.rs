use std::collections::HashMap;

pub struct PageOrderingRules {
    rules: HashMap<u32, Vec<u32>>,
}

impl PageOrderingRules {
    pub fn from_string(input: &str) -> Self {
        let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

        input.lines().for_each(|line| {
            let mut components = line.split("|");
            let index: u32 = components.nth(0).unwrap().parse().unwrap();
            let rule: u32 = components.nth(0).unwrap().parse().unwrap();

            let mut index_rules: Vec<u32> = Vec::from([rule]);

            match rules.get(&index) {
                Some(current) => {
                    index_rules.append(&mut current.to_vec());
                }
                _ => {}
            };

            rules.insert(index, index_rules);
        });

        Self { rules }
    }
}

impl PageOrderingRules {
    pub fn is_correct_order(&self, print_order: &Vec<u32>) -> bool {
        print_order.iter().enumerate().all(|(index, entry)| {
            let rules = self.rules.get(entry);

            if rules.is_none() {
                return true;
            }

            let rules = rules.unwrap();

            print_order
                .iter()
                .take(index)
                .all(|number| !rules.contains(number))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_correctly_ordered_print() {
        let rules_input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13";
        let print_line = Vec::from([75, 47, 61, 53, 29]);

        let rules = PageOrderingRules::from_string(rules_input);
        let result = rules.is_correct_order(&print_line);

        assert_eq!(result, true);
    }
}
