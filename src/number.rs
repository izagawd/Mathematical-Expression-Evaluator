use std::ops::{Add, Div, Mul, Sub};


impl Add for Number{
    type Output = Number;
    fn add(self, other: Number) -> Number{
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a + b),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f32 + b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a + b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a + b as f32),
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a - b),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f32 - b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a - b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a - b as f32),
        }
    }
}

impl Mul for Number {
    type Output = Number;

    fn mul(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a * b),
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f32 * b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a * b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a * b as f32),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Number{
    Int(i32),
    Float(f32),
}
impl Div for Number {
    type Output = Number;

    fn div(self, other: Number) -> Number {
        match (self, other) {
            (Number::Int(a), Number::Int(b)) => Number::Int(a / b), // Integer division
            (Number::Int(a), Number::Float(b)) => Number::Float(a as f32 / b),
            (Number::Float(a), Number::Float(b)) => Number::Float(a / b),
            (Number::Float(a), Number::Int(b)) => Number::Float(a / b as f32),
        }
    }
}