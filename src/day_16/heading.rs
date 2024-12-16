use crate::util::{Direction, Position};

pub struct Heading {
    pub steps: usize,
    pub turns: usize,
    pub position: Position,
    pub direction: Direction,
}

impl Heading {
    pub fn cost(&self) -> usize {
        self.steps + (self.turns * 1000)
    }

    fn priority(&self) -> usize {
        self.cost()
    }

    pub fn new_with(&self, position: Position, direction: Direction) -> Self {
        let steps = self.steps + 1;
        let turns = match self.direction == direction {
            true => self.turns,
            _ => self.turns + 1,
        };

        Self {
            position,
            direction,
            steps,
            turns,
        }
    }
}

impl PartialEq for Heading {
    fn eq(&self, other: &Self) -> bool {
        self.priority() == other.priority()
    }
}

impl Eq for Heading {}

impl PartialOrd for Heading {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority().partial_cmp(&(self.priority()))
    }
}

impl Ord for Heading {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&(other.priority()))
    }
}

impl From<(Position, Direction)> for Heading {
    fn from(value: (Position, Direction)) -> Self {
        Self {
            position: value.0,
            direction: value.1,
            steps: 0,
            turns: 0,
        }
    }
}
