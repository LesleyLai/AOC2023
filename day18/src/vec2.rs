use std::ops::{Add, Mul, Sub};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Vec2 {
    pub x: isize,
    pub y: isize,
}

impl Vec2 {
    pub fn new(x: isize, y: isize) -> Self {
        Vec2 { x, y }
    }
}

impl From<(isize, isize)> for Vec2 {
    fn from((x, y): (isize, isize)) -> Self {
        Vec2::new(x, y)
    }
}

impl From<Vec2> for (isize, isize) {
    fn from(point: Vec2) -> Self {
        (point.x, point.y)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}

impl Mul<isize> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: isize) -> Self::Output {
        (self.x * rhs, self.y * rhs).into()
    }
}
