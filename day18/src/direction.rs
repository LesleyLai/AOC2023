use crate::vec2::Vec2;

use Direction::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for (isize, isize) {
    fn from(dir: Direction) -> Self {
        match dir {
            Up => (0, -1),
            Down => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        }
    }
}

impl From<Direction> for Vec2 {
    fn from(dir: Direction) -> Self {
        let pair: (isize, isize) = dir.into();
        pair.into()
    }
}
