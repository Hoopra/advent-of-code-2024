mod robot;

use robot::Robot;

use crate::util::read_input;

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

pub fn solve_part_2() -> usize {
    let _input = read_input("src/day_14/input.txt");

    0
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
