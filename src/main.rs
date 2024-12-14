mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
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
            println!("part 1: {}", day_01::solve_part_1());
            println!("part 2: {}", day_01::solve_part_2());
        }
        2 => {
            println!("part 1: {}", day_02::solve_part_1());
            println!("part 2: {}", day_02::solve_part_2());
        }
        3 => {
            println!("part 1: {}", day_03::solve_part_1());
            println!("part 2: {}", day_03::solve_part_2());
        }
        4 => {
            println!("part 1: {}", day_04::solve_part_1());
            println!("part 2: {}", day_04::solve_part_2());
        }
        5 => {
            println!("part 1: {}", day_05::solve_part_1());
            println!("part 2: {}", day_05::solve_part_2());
        }
        6 => {
            println!("part 1: {}", day_06::solve_part_1());
            println!("part 2: {}", day_06::solve_part_2());
        }
        7 => {
            println!("part 1: {}", day_07::solve_part_1());
            println!("part 2: {}", day_07::solve_part_2());
        }
        8 => {
            println!("part 1: {}", day_08::solve_part_1());
            println!("part 2: {}", day_08::solve_part_2());
        }
        9 => {
            let result = day_09::solve_part_1();
            assert_eq!(result, 6242766523059);
            println!("part 1: {}", result);

            let result = day_09::solve_part_2();
            assert_eq!(result, 6272188244509);
            println!("part 2: {}", result);
        }
        10 => {
            println!("part 1: {}", day_10::solve_part_1());
            println!("part 2: {}", day_10::solve_part_2());
        }
        11 => {
            println!("part 1: {}", day_11::solve_part_1());
            println!("part 2: {}", day_11::solve_part_2());
        }
        12 => {
            println!("part 1: {}", day_12::solve_part_1());
            println!("part 2: {}", day_12::solve_part_2());
        }
        _ => {
            println!("not yet solved");
        }
    }
}
