#![allow(dead_code)]

use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vector2(pub i64, pub i64);

impl Vector2 {
    pub const ZERO: Vector2 = Vector2(0, 0);

    pub fn new(x: i64, y: i64) -> Vector2 { Vector2(x, y) }

    pub fn manhattan_distance(&self, other: &Vector2) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            0: self.0 - other.0,
            1: self.1 - other.1,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl Mul<i64> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self {
        Self {
            0: self.0 * rhs,
            1: self.1 * rhs,
        }
    }
}

#[cfg(test)]
mod tests_util_vector2 {
    use crate::util::Vector2;

    #[test]
    fn manhattan_distance() {
        assert_eq!(2, Vector2(1, 1).manhattan_distance(&Vector2(2, 2)));
        assert_eq!(4, Vector2(0, 0).manhattan_distance(&Vector2(2, 2)));
        assert_eq!(6, Vector2(-1, -1).manhattan_distance(&Vector2(2, 2)));
    }

    #[test]
    fn add() {
        assert_eq!(Vector2(-1, -1), Vector2(-1, -1) + Vector2(0, 0));
        assert_eq!(Vector2(-1, 0), Vector2(-1, -1) + Vector2(0, 1));
        assert_eq!(Vector2(0, 1), Vector2(-1, 0) + Vector2(1, 1));
        assert_eq!(Vector2(-2, -1), Vector2(0, 1) + Vector2(-2, -2));
    }

    #[test]
    fn add_assign() {
        let mut v = Vector2(-1, -1);
        v += Vector2(0, 0);
        assert_eq!(Vector2(-1, -1), v);
        v += Vector2(0, 1);
        assert_eq!(Vector2(-1, 0), v);
        v += Vector2(1, 1);
        assert_eq!(Vector2(0, 1), v);
        v += Vector2(-2, -2);
        assert_eq!(Vector2(-2, -1), v);
    }

    #[test]
    fn sub() {
        assert_eq!(Vector2(-1, -1), Vector2(-1, -1) - Vector2(0, 0));
        assert_eq!(Vector2(-1, -2), Vector2(-1, -1) - Vector2(0, 1));
        assert_eq!(Vector2(-2, -1), Vector2(-1, 0) - Vector2(1, 1));
        assert_eq!(Vector2(2, 3), Vector2(0, 1) - Vector2(-2, -2));
    }
}