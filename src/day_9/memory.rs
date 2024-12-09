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
}
