mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
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
        3 => {
            println!("part 1: {}", day_3::solve_part_1());
            println!("part 2: {}", day_3::solve_part_2());
        }
        4 => {
            println!("part 1: {}", day_4::solve_part_1());
            println!("part 2: {}", day_4::solve_part_2());
        }
        5 => {
            println!("part 1: {}", day_5::solve_part_1());
            println!("part 2: {}", day_5::solve_part_2());
        }
        6 => {
            println!("part 1: {}", day_6::solve_part_1());
            println!("part 2: {}", day_6::solve_part_2());
        }
        7 => {
            println!("part 1: {}", day_7::solve_part_1());
            println!("part 2: {}", day_7::solve_part_2());
        }
        8 => {
            println!("part 1: {}", day_8::solve_part_1());
            println!("part 2: {}", day_8::solve_part_2());
        }
        9 => {
            println!("part 1: {}", day_9::solve_part_1());
            println!("part 2: {}", day_9::solve_part_2());
        }
        _ => {
            println!("not yet solved");
        }
    }
}
