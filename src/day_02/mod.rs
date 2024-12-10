mod report;

use crate::util::read_input;
use report::reports_from_text;

fn find_safe_reports(input: String) -> usize {
    let reports = reports_from_text(input);
    reports.iter().filter(|report| report.is_safe()).count()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_2/input.txt");

    find_safe_reports(input)
}

fn find_safe_reports_with_dampener(input: String) -> usize {
    let reports = reports_from_text(input);
    reports
        .iter()
        .filter(|report| report.is_safe_with_dampener())
        .count()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_2/input.txt");

    find_safe_reports_with_dampener(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_safe_reports() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let result = find_safe_reports(input.to_string());

        assert_eq!(result, 2);
    }

    #[test]
    fn finds_safe_reports_with_dampener() {
        let input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let result = find_safe_reports_with_dampener(input.to_string());

        assert_eq!(result, 4);
    }
}
