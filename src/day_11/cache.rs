use std::collections::HashMap;

use super::number::Number;

pub struct NumberCache {
    entries: HashMap<String, Vec<Number>>,
    count: HashMap<u64, usize>,
}

impl NumberCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            count: HashMap::new(),
        }
    }
}

impl NumberCache {
    fn key(value: u64, blinks: usize) -> String {
        format!("{}.{}", value, blinks)
    }

    pub fn get(&self, value: u64, blinks: usize) -> Option<&Vec<Number>> {
        let key = NumberCache::key(value, blinks);

        self.entries.get(&key)
    }

    pub fn set(&mut self, value: u64, blinks: usize, entries: Vec<Number>) {
        let key = NumberCache::key(value, blinks);

        self.entries.insert(key, entries);
    }

    pub fn get_count(&self, value: u64) -> Option<&usize> {
        self.count.get(&value)
    }

    pub fn set_count(&mut self, value: u64, count: usize) {
        self.count.insert(value, count);
    }
}
