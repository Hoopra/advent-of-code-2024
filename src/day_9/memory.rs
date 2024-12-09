#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MemoryEntry {
    pub id: usize,
    pub length: usize,
    pub indices: Vec<usize>,
}

impl MemoryEntry {
    pub fn new(id: usize, length: usize, indices: Vec<usize>) -> Self {
        Self {
            id,
            length,
            indices,
        }
    }
}

impl MemoryEntry {
    pub fn checksum(&self) -> usize {
        self.indices.iter().map(|index| index * self.id).sum()
    }
}

pub enum MemorySlotType {
    Data,
    Empty,
}

impl MemorySlotType {
    pub fn reverse(&self) -> Self {
        match self {
            MemorySlotType::Data => MemorySlotType::Empty,
            MemorySlotType::Empty => MemorySlotType::Data,
        }
    }
}

#[derive(Debug, Default)]
pub struct Memory {
    unordered: Vec<MemoryEntry>,
    ordered: Vec<MemoryEntry>,
    empty: Vec<usize>,
}

impl Memory {
    pub fn from_string(input: &str) -> Self {
        let mut memory = Memory::default();

        let mut block_type = MemorySlotType::Data;
        let mut index: usize = 0;
        let mut id: usize = 0;

        input.chars().for_each(|entry: char| {
            let length = entry.to_digit(10).unwrap() as usize;

            let mut indices: Vec<usize> = (index..(index + length)).collect();

            match block_type {
                MemorySlotType::Data => {
                    let entry = MemoryEntry::new(id, length, indices);
                    memory.unordered.push(entry);

                    id += 1;
                }
                MemorySlotType::Empty => {
                    memory.empty.append(&mut indices);
                }
            }

            index += length;
            block_type = block_type.reverse();
        });

        memory
    }
}

impl Memory {
    pub fn order(&mut self) {
        while let Some(next) = self.unordered.pop() {
            let length = next.length;

            let mut empty: Vec<usize> = self.empty.clone();
            empty.append(&mut next.indices.clone());
            empty.sort();

            let new_indices = empty.clone().into_iter().take(length).collect();
            self.empty = empty.into_iter().skip(length).collect();

            self.ordered
                .push(MemoryEntry::new(next.id, length, new_indices));
        }
    }

    fn find_continuous_space(&self, length: usize) -> Option<usize> {
        if length == 1 {
            return Some(0);
        }

        let mut matched: usize = 1;

        let total = self.empty.len();
        for index in 0..total {
            let start = self.empty.get(index).unwrap();

            for next_index in 1..length {
                let next = self.empty.get(index + next_index);
                if next.is_none() {
                    matched = 1;
                    break;
                }

                match next {
                    Some(next) if next == &(start + next_index) => {
                        matched += 1;

                        if matched == length {
                            return Some(index);
                        }
                    }
                    _ => {
                        matched = 1;
                        break;
                    }
                }
            }
        }

        None
    }

    pub fn order_whole_files(&mut self) {
        while let Some(next) = self.unordered.pop() {
            let length = next.length;

            let index = self.find_continuous_space(length);
            if index.is_none() {
                self.ordered.push(next);
                continue;
            }

            let index = index.unwrap();
            println!("for id {} (l = {}) index {}", next.id, length, index,);

            let mut empty: Vec<usize> = self.empty.clone();

            let new_indices: Vec<usize> =
                empty.clone().into_iter().skip(index).take(length).collect();

            empty.append(&mut next.indices.clone());
            empty.sort();

            println!("new_indices {:?}", new_indices);

            self.empty = empty
                .into_iter()
                .filter(|index| !new_indices.contains(index))
                .collect();

            self.ordered
                .push(MemoryEntry::new(next.id, length, new_indices));
        }
    }

    pub fn checksum(&self) -> usize {
        self.ordered.iter().map(|entry| entry.checksum()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_memory() {
        let input = "12345";
        let memory = Memory::from_string(input);

        assert_eq!(memory.empty, Vec::from([1, 2, 6, 7, 8, 9]));
        assert_eq!(memory.ordered, Vec::from([]));
        assert_eq!(
            memory.unordered,
            Vec::from([
                MemoryEntry {
                    id: 0,
                    length: 1,
                    indices: Vec::from([0])
                },
                MemoryEntry {
                    id: 1,
                    length: 3,
                    indices: Vec::from([3, 4, 5])
                },
                MemoryEntry {
                    id: 2,
                    length: 5,
                    indices: Vec::from([10, 11, 12, 13, 14])
                }
            ])
        );
    }

    #[test]
    fn sorts_memory() {
        let input = "12345";
        let mut memory = Memory::from_string(input);
        memory.order();

        assert_eq!(memory.empty, Vec::from([9, 10, 11, 12, 13, 14]));
    }

    #[test]
    fn sorts_memory_complex_case() {
        let input = "2333133121414131402";
        let mut memory = Memory::from_string(input);

        memory.order();

        assert_eq!(
            memory.empty,
            Vec::from([28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41])
        );
    }

    #[test]
    fn finds_continuous_space_with_length() {
        let memory = Memory::from_string("2333133121414131402");

        let result = memory.find_continuous_space(3);
        assert_eq!(result, Some(0));

        let memory = Memory::from_string("12345");

        let result = memory.find_continuous_space(3);
        assert_eq!(result, Some(2));

        let result = memory.find_continuous_space(4);
        assert_eq!(result, Some(2));

        let result = memory.find_continuous_space(5);
        assert_eq!(result, None);
    }

    #[test]
    fn sorts_memory_by_whole_files() {
        let input = "2333133121414131402";
        let mut memory = Memory::from_string(input);

        memory.order_whole_files();

        println!("ordered: {:?}", memory.ordered);

        assert_eq!(
            memory.empty,
            Vec::from([11, 14, 18, 19, 20, 21, 26, 31, 32, 33, 34, 35])
        );

        // [
        // MemoryEntry { id: 0, length: 2, indices: [0, 1] }
        // MemoryEntry { id: 9, length: 2, indices: [2, 3] },
        // MemoryEntry { id: 2, length: 1, indices: [4] },
        // MemoryEntry { id: 1, length: 3, indices: [7, 11, 14] },
        // MemoryEntry { id: 8, length: 4, indices: [36, 37, 38, 39] },
        // MemoryEntry { id: 7, length: 3, indices: [8, 9, 10] },
        // MemoryEntry { id: 6, length: 4, indices: [27, 28, 29, 30] },
        // MemoryEntry { id: 5, length: 4, indices: [23, 24, 25, 26] },
        // MemoryEntry { id: 4, length: 2, indices: [12, 13] },
        // MemoryEntry { id: 3, length: 3, indices: [15, 16, 17] },
        // ]

        // 00...111...2...333.44.5555.6666.777.888899
        // 0099.111...2...333.44.5555.6666.777.8888..
        // 0099.1117772...333.44.5555.6666.....8888..
        // 0099.111777244.333....5555.6666.....8888..
        // 00992111777.44.333....5555.6666.....8888..
    }
}
