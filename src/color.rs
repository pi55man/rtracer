use std::fmt::Debug;

use crate::vec::Vec3;

pub struct Color {
    pub r: u64,
    pub g: u64,
    pub b: u64,
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}\n", self.r, self.g, self.b)
    }
}
