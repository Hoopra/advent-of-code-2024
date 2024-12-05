use std::collections::HashMap;

pub struct PageOrderingRules {
    after_rules: HashMap<u32, Vec<u32>>,
}

impl PageOrderingRules {
    pub fn from_string(input: &str) -> Self {
        let mut after_rules: HashMap<u32, Vec<u32>> = HashMap::new();

        input.lines().for_each(|line| {
            let mut components = line.split("|");
            let index: u32 = components.nth(0).unwrap().parse().unwrap();
            let rule: u32 = components.nth(0).unwrap().parse().unwrap();

            let mut index_rules: Vec<u32> = Vec::from([rule]);

            match after_rules.get(&index) {
                Some(current) => {
                    index_rules.append(&mut current.to_vec());
                }
                _ => {}
            };

            after_rules.insert(index, index_rules);
        });

        Self { after_rules }
    }
}

impl PageOrderingRules {
    fn is_index_valid(&self, print_order: &Vec<u32>, index: usize) -> bool {
        let entry = print_order.get(index).unwrap();
        let rules = self.after_rules.get(entry);

        if rules.is_none() {
            return true;
        }

        let rules = rules.unwrap();

        print_order
            .iter()
            .take(index)
            .all(|number| !rules.contains(number))
    }

    pub fn is_correct_order(&self, print_order: &Vec<u32>) -> bool {
        print_order
            .iter()
            .enumerate()
            .all(|(index, _)| self.is_index_valid(print_order, index))
    }

    fn fits_at_index(&self, print_order: &Vec<u32>, value: u32, index: usize) -> bool {
        let mut builder: Vec<u32> = Vec::new();

        let first = &print_order.clone()[0..index];
        let last = &print_order.clone()[index..print_order.len() - 1];

        builder.append(&mut Vec::from(first));
        builder.push(value);
        builder.append(&mut Vec::from(last));

        self.is_correct_order(&builder)
    }

    pub fn order_correctly(&self, print_order: &Vec<u32>) -> Vec<u32> {
        let mut result = Vec::new();
        let mut temporary: Vec<u32> = Vec::new();

        print_order.iter().enumerate().for_each(|(index, value)| {
            if self.is_index_valid(print_order, index) {
                result.push(*value);
                return;
            }

            temporary.push(*value);
        });

        while let Some(next) = temporary.pop() {
            for i in 0..result.len() {
                if self.fits_at_index(&result, next, i) {
                    result.insert(i, next);
                    break;
                }
            }
        }

        result
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

    #[test]
    fn correctly_orders_print() {
        let rules_input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13";
        let rules = PageOrderingRules::from_string(rules_input);

        let print_line = Vec::from([75, 97, 47, 61, 53]);
        let result = rules.order_correctly(&print_line);
        assert_eq!(result, Vec::from([97, 75, 47, 61, 53]));

        let print_line = Vec::from([61, 13, 29]);
        let result = rules.order_correctly(&print_line);
        assert_eq!(result, Vec::from([61, 29, 13]));

        let print_line = Vec::from([97, 13, 75, 29, 47]);
        let result = rules.order_correctly(&print_line);
        assert_eq!(result, Vec::from([97, 75, 47, 29, 13]));
    }
}
