use super::Direction;

pub type Position = (isize, isize);

pub fn move_steps_in_direction<'a>(
    position: &'a Position,
    steps: isize,
    direction: &'a Direction,
) -> Position {
    let step = direction.step_2d();

    let position = (position.0 + steps * step.0, position.1 + steps * step.1);
    position
}
