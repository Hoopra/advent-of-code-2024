mod memory;

use crate::util::read_input;
use memory::Memory;

fn order_memory_checksum(input: &str) -> usize {
    let mut memory = Memory::from_string(input);

    memory.order(false);
    memory.checksum()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_09/input.txt");

    // TODO: optimise
    order_memory_checksum(&input)
}

fn whole_file_order_memory_checksum(input: &str) -> usize {
    let mut memory = Memory::from_string(input);

    memory.order(true);
    memory.checksum()
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_09/input.txt");

    // TODO: optimise
    whole_file_order_memory_checksum(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_ordered_memory_checksum() {
        let input = "2333133121414131402";
        let result = order_memory_checksum(input);

        assert_eq!(result, 1928);
    }

    #[test]
    fn finds_whole_file_ordered_memory_checksum() {
        let input = "2333133121414131402";
        let result = whole_file_order_memory_checksum(input);

        assert_eq!(result, 2858);
    }
}
