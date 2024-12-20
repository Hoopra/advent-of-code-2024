use crate::util::Position;

#[derive(Debug)]
pub struct Path {
    pub position: Position,
    pub distance: isize,
}

impl Path {
    pub fn new(position: Position, distance: isize) -> Self {
        Self { position, distance }
    }
}

impl Path {
    fn priority(&self) -> isize {
        self.distance
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.priority() == other.priority()
    }
}

impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority().partial_cmp(&(self.priority()))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
