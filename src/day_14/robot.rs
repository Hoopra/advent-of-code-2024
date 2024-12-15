use crate::util::Position;

pub struct Robot {
    pub position: Position,
    velocity: Position,
}

fn coordinates_from_string(input: &str) -> Position {
    let mut coordinates = input.split("=").nth(1).unwrap().split(",");
    (
        coordinates.nth(0).unwrap().parse().unwrap(),
        coordinates.nth(0).unwrap().parse().unwrap(),
    )
}

impl Robot {
    pub fn from_string(input: &str) -> Self {
        let mut components = input.split(" ");
        let position_string = components.nth(0).unwrap();
        let velocity_string = components.nth(0).unwrap();

        Self {
            position: coordinates_from_string(position_string),
            velocity: coordinates_from_string(velocity_string),
        }
    }
}

fn find_new_coordinate(start: isize, velocity: isize, wrap: isize, times: isize) -> isize {
    let new_value = (start + ((times as isize) * velocity)) % wrap;

    match new_value < 0 {
        false => new_value,
        true => wrap + new_value,
    }
}

impl Robot {
    pub fn move_times(&mut self, space: (isize, isize), times: usize) {
        let (x, y) = self.position;
        let (v_x, v_y) = self.velocity;

        self.position = (
            find_new_coordinate(x, v_x, space.0, times as isize),
            find_new_coordinate(y, v_y, space.1, times as isize),
        )
    }

    pub fn find_quadrant(&self, space: (isize, isize)) -> Option<usize> {
        let (x, y) = self.position;

        let b_x_half = space.0 / 2;
        let b_y_half = space.1 / 2;

        if x == b_x_half || y == b_y_half {
            return None;
        }

        let result = match x > b_x_half {
            false => match y > b_y_half {
                true => 4,
                false => 2,
            },
            true => match y > b_y_half {
                true => 3,
                false => 1,
            },
        };

        // ..... 2..1.
        // ..... .....
        // 1.... .....

        // ..... .....
        // ...12 .....
        // .1... 1....

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Robot;

    #[test]
    fn calculates_position_after_5_seconds() {
        let input = "p=2,4 v=2,-3";
        let space = (11, 7);

        let mut robot = Robot::from_string(input);
        assert_eq!(robot.position, (2, 4));
        assert_eq!(robot.velocity, (2, -3));

        robot.move_times(space, 5);

        assert_eq!(robot.position, (1, 3));
        assert_eq!(robot.find_quadrant(space), None);
    }
}
