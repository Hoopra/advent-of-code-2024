mod day_1;
mod day_2;
mod util;

use std::io::stdin;

fn main() {
    println!("solve for day: ");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let day = buffer.trim().parse().expect("type a valid number");

    println!("day {day}");
    match day {
        1 => {
            println!("part 1: {}", day_1::solve_part_1());
            println!("part 2: {}", day_1::solve_part_2());
        }
        2 => {
            println!("part 1: {}", day_2::solve_part_1());
            println!("part 2: {}", day_2::solve_part_2());
        }
        _ => {
            println!("not yet solved");
        }
    }
}
