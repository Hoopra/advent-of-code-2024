mod cache;
mod collection;
mod number;

use crate::util::read_input;
use cache::NumberCache;
use collection::NumberCollection;

pub fn arrangement_after_blinks(input: &str, blinks: u64) -> usize {
    let mut collection = NumberCollection::from_string(input);

    let mut result = 0;

    for _ in 0..blinks {
        collection.blink();
        result = collection.count();
    }

    result
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_11/input.txt");

    arrangement_after_blinks(&input, 25)
}

pub fn arrangement_after_blinks_with_cache(input: &str, blinks: usize, steps: f64) -> usize {
    let mut cache = NumberCache::new();
    let mut collection = NumberCollection::from_string(input);

    collection.blink_times(blinks, steps, &mut cache)
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_11/input.txt");

    arrangement_after_blinks_with_cache(&input, 75, 3.0)
}
