mod ordering;

use ordering::PageOrderingRules;

use crate::util::read_input;

fn find_middle_index(length: usize) -> usize {
    (0.5 * length as f64).floor() as usize
}

fn text_to_numbers(text: &str) -> Vec<u32> {
    text.split(",")
        .map(|entry| entry.parse().unwrap())
        .collect()
}

fn verify_page_ordering(input: String) -> u32 {
    let mut components = input.split("\n\n");
    let rules_text = components.nth(0).unwrap();
    let print_text = components.nth(0).unwrap();

    let rules = PageOrderingRules::from_string(rules_text);

    print_text
        .lines()
        .filter_map(|line| {
            let print_order: Vec<u32> = text_to_numbers(line);

            match rules.is_correct_order(&print_order) {
                true => Some(print_order[find_middle_index(print_order.len())]),
                false => None,
            }
        })
        .sum()
}

fn correct_page_ordering(input: String) -> u32 {
    let mut components = input.split("\n\n");
    let rules_text = components.nth(0).unwrap();
    let print_text = components.nth(0).unwrap();

    let rules = PageOrderingRules::from_string(rules_text);

    print_text
        .lines()
        .map(|line| {
            let print_order: Vec<u32> = text_to_numbers(line);

            if rules.is_correct_order(&print_order) {
                return 0;
            }

            let print_order = rules.order_correctly(&print_order);
            print_order[find_middle_index(print_order.len())]
        })
        .sum()
}

pub fn solve_part_1() -> u32 {
    let input = read_input("src/day_5/input.txt");

    verify_page_ordering(input)
}

pub fn solve_part_2() -> u32 {
    let input = read_input("src/day_5/input.txt");

    correct_page_ordering(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifies_correct_print_orders() {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let result = verify_page_ordering(input.to_string());

        assert_eq!(result, 143);
    }

    #[test]
    fn corrects_print_orders() {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let result = correct_page_ordering(input.to_string());

        assert_eq!(result, 123);
    }
}
