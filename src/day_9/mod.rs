mod memory;

use crate::util::read_input;
use memory::Memory;

fn ordered_memory_checksum(input: &str) -> usize {
    let mut memory = Memory::from_string(input);

    memory.order();
    memory.checksum()
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_9/input.txt");

    ordered_memory_checksum(&input)
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_9/input.txt");

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_ordered_memory_checksum() {
        let input = "2333133121414131402";
        let result = ordered_memory_checksum(input);

        assert_eq!(result, 1928);
    }
}
