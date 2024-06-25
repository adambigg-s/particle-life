


use std::ops::{AddAssign, Mul};



#[derive(Clone, Debug, Copy)]
pub struct Vector<Type> {
    pub x: Type,
    pub y: Type,
}

impl AddAssign for Vector<f32> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Mul<f32> for Vector<f32> {
    type Output = Vector<f32>;
    fn mul(self, rhs: f32) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
