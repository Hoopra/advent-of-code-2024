use std::collections::HashMap;

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
    empty: HashMap<usize, ()>,
}

impl Memory {
    pub fn empty_keys(&self) -> Vec<&usize> {
        let mut keys: Vec<&usize> = self.empty.keys().into_iter().collect();
        keys.sort();

        keys
    }

    pub fn from_string(input: &str) -> Self {
        let mut memory = Memory::default();

        let mut block_type = MemorySlotType::Data;
        let mut index: usize = 0;
        let mut id: usize = 0;

        input.chars().for_each(|entry: char| {
            let length = entry.to_digit(10).unwrap() as usize;

            let indices: Vec<usize> = (index..(index + length)).collect();

            match block_type {
                MemorySlotType::Data => {
                    let entry = MemoryEntry::new(id, length, indices);
                    memory.unordered.push(entry);

                    id += 1;
                }
                MemorySlotType::Empty => {
                    for index in indices {
                        memory.empty.insert(index, ());
                    }
                }
            }

            index += length;
            block_type = block_type.reverse();
        });

        memory
    }
}

impl Memory {
    fn add_empty_indices(&mut self, indices: &Vec<usize>) {
        for index in indices {
            self.empty.insert(*index, ());
        }
    }

    fn remove_empty_indices(&mut self, indices: &Vec<usize>) {
        for index in indices {
            self.empty.remove(&index);
        }
    }

    fn find_empty_indices(&self, length: usize, continuous: bool) -> Option<Vec<usize>> {
        let mut result = Vec::new();

        let keys = self.empty_keys();

        for key in keys.iter() {
            result.push(**key);

            if continuous && !is_continuous(&result) {
                result.clear();
                result.push(**key);
                continue;
            }

            if result.len() == length {
                return Some(result);
            }
        }

        None
    }

    pub fn order(&mut self, continuous: bool) {
        while let Some(next) = self.unordered.pop() {
            let length = next.length;

            if !continuous {
                self.add_empty_indices(&next.indices);
            }

            let new_indices = self.find_empty_indices(length, continuous);
            let first_index = next.indices.first().unwrap();

            match new_indices {
                Some(indices) if indices.first().unwrap() <= first_index => {
                    self.remove_empty_indices(&indices);

                    if continuous {
                        self.add_empty_indices(&next.indices);
                    }

                    let entry = MemoryEntry::new(next.id, length, indices);
                    self.ordered.push(entry);
                }
                _ => self.ordered.push(next),
            }
        }
    }

    pub fn checksum(&self) -> usize {
        self.ordered.iter().map(|entry| entry.checksum()).sum()
    }
}

fn is_continuous(array: &Vec<usize>) -> bool {
    if array.len() <= 1 {
        return true;
    }

    let first = array.first().unwrap();

    array
        .iter()
        .skip(1)
        .enumerate()
        .all(|(index, entry)| entry == &(index + first + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_memory() {
        let input = "12345";
        let memory = Memory::from_string(input);

        assert_eq!(
            memory.empty,
            HashMap::from([(1, ()), (2, ()), (6, ()), (7, ()), (8, ()), (9, ())])
        );
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
        memory.order(false);

        assert_eq!(
            memory.empty_keys(),
            Vec::from([&9, &10, &11, &12, &13, &14])
        );
    }

    #[test]
    fn sorts_memory_complex_case() {
        let input = "2333133121414131402";
        let mut memory = Memory::from_string(input);

        memory.order(false);

        assert_eq!(
            memory.empty_keys(),
            Vec::from([&28, &29, &30, &31, &32, &33, &34, &35, &36, &37, &38, &39, &40, &41])
        );
    }

    #[test]
    fn finds_continuous_space_with_length() {
        let memory = Memory::from_string("2333133121414131402");

        let result = memory.find_empty_indices(3, true);
        assert_eq!(result, Some(Vec::from([2, 3, 4])));

        let memory = Memory::from_string("12345");

        let result = memory.find_empty_indices(3, true);
        assert_eq!(result, Some(Vec::from([6, 7, 8])));

        let result = memory.find_empty_indices(4, true);
        assert_eq!(result, Some(Vec::from([6, 7, 8, 9])));

        let result = memory.find_empty_indices(5, true);
        assert_eq!(result, None);
    }

    #[test]
    fn sorts_memory_by_whole_files() {
        let input = "2333133121414131402";
        let mut memory = Memory::from_string(input);

        memory.order(true);

        assert_eq!(
            memory.empty_keys(),
            Vec::from([&11, &14, &18, &19, &20, &21, &26, &31, &32, &33, &34, &35, &40, &41])
        );
    }
}
