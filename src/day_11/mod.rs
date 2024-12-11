mod stone;
use stone::StoneCollection;

use crate::util::read_input;

const STONE_CUTOFF: usize = 50000;

pub fn collection_arrangement_after_blinks(
    mut collection: StoneCollection,
    blinks: u64,
    total: u64,
) -> usize {
    let mut result = 0;

    for i in 0..blinks {
        println!("blink {} / {} (total {})", i, blinks, total);

        collection.blink();
        result = collection.count();

        if collection.count() >= STONE_CUTOFF {
            let collections = collection.split();

            for collection in collections {
                result +=
                    collection_arrangement_after_blinks(collection, blinks - i - 1, total + 2);
            }

            return result;
        }
    }

    result
}

pub fn arrangement_after_blinks(input: &str, blinks: u64) -> usize {
    let collection = StoneCollection::from_string(input);

    collection_arrangement_after_blinks(collection, blinks, 0)
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_11/input.txt");

    arrangement_after_blinks(&input, 25)
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_11/input.txt");

    arrangement_after_blinks(&input, 75)
}
