mod robot;

use crate::util::{read_input, Position};
use robot::Robot;
use std::collections::HashSet;

fn print_map(space: (isize, isize), robot_positions: HashSet<Position>) {
    println!("");

    for y in 0..=space.1 {
        println!("");
        for x in 0..=space.0 {
            match robot_positions.contains(&(x, y)) {
                true => print!("#"),
                false => print!("."),
            }
        }
    }
}

fn find_safety_score(input: &str, space: (isize, isize), times: usize) -> usize {
    let quadrant_counts = input
        .lines()
        .map(|text| {
            let mut robot = Robot::from_string(text);
            robot.move_times(space, times);

            robot.find_quadrant(space)
        })
        .fold((0, 0, 0, 0), |result, quadrant| match quadrant {
            None => result,
            Some(quadrant) => {
                let (q1, q2, q3, q4) = result;
                (
                    if quadrant == 1 { q1 + 1 } else { q1 },
                    if quadrant == 2 { q2 + 1 } else { q2 },
                    if quadrant == 3 { q3 + 1 } else { q3 },
                    if quadrant == 4 { q4 + 1 } else { q4 },
                )
            }
        });

    quadrant_counts.0 * quadrant_counts.1 * quadrant_counts.2 * quadrant_counts.3
}

pub fn solve_part_1() -> usize {
    let input = read_input("src/day_14/input.txt");

    find_safety_score(&input, (101, 103), 100)
}

fn are_positions_lined_up(positions: &HashSet<Position>, line_length: usize) -> bool {
    let mut length = 0;

    for (x_0, y) in positions {
        let mut x = x_0 + 1;
        while positions.contains(&(x, *y)) {
            x += 1;
            length += 1;
        }

        if (length + 1) >= line_length {
            return true;
        }

        length = 0;
    }

    false
}

fn find_moves_for_easter_egg(input: &str, space: (isize, isize)) -> usize {
    let mut robots: Vec<Robot> = input.lines().map(|text| Robot::from_string(text)).collect();

    let mut robot_references: Vec<&mut Robot> = robots.iter_mut().collect();
    let mut positions: HashSet<Position> = HashSet::new();
    let mut times = 0;

    loop {
        positions.clear();
        times += 1;

        for robot in robot_references.iter_mut() {
            robot.move_times(space, 1);
            positions.insert(robot.position);
        }

        if are_positions_lined_up(&positions, 10) {
            break;
        }
    }

    print_map(space, positions);

    times
}

pub fn solve_part_2() -> usize {
    let input = read_input("src/day_14/input.txt");

    find_moves_for_easter_egg(&input, (101, 103))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_safety_score() {
        let input = "p=0,4 v=3,-3\np=6,3 v=-1,-3\np=10,3 v=-1,2\np=2,0 v=2,-1\np=0,0 v=1,3\np=3,0 v=-2,-2\np=7,6 v=-1,-3\np=3,0 v=-1,-2\np=9,3 v=2,3\np=7,3 v=-1,2\np=2,4 v=2,-3\np=9,5 v=-3,-3";

        let result = find_safety_score(input, (11, 7), 100);
        assert_eq!(result, 12);
    }
}
