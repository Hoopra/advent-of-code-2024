use std::collections::HashMap;

use crate::util::Position;

pub struct Map2D {
    tiles: HashMap<Position, char>,
}
