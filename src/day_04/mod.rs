mod searcher;

use crate::util::read_input;

fn find_xmas(input: String) -> u32 {
    searcher::find_instances(input, "XMAS")
}

pub fn solve_part_1() -> u32 {
    let input = read_input("src/day_04/input.txt");

    find_xmas(input)
}

fn find_cross_mas(input: String) -> u32 {
    searcher::find_cross_instances(input, "MAS")
}

pub fn solve_part_2() -> u32 {
    let input = read_input("src/day_04/input.txt");

    find_cross_mas(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_xmas_in_short_text() {
        let sample = "..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....";

        let result = find_xmas(sample.to_string());
        assert_eq!(result, 4);
    }

    #[test]
    fn finds_xmas_in_sample_text() {
        let sample = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

        let result = find_xmas(sample.to_string());
        assert_eq!(result, 18);
    }

    #[test]
    fn finds_x_mas_in_sample_text() {
        let sample = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

        let result = find_cross_mas(sample.to_string());
        assert_eq!(result, 9);
    }
}
