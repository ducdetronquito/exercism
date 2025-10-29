// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::ops::Rem;

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate().filter(|(i, _ )| i % 2 == 0).map(|(_, x)| x)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        let Position(x1, y1) = Position(0, 0);
        let Position(x2, y2) = self;
        (x1 - x2).abs() + (y1 -y2).abs()
    }
}
