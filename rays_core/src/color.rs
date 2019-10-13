use std::ops::{Add, Sub, Mul};
use std::cmp::PartialEq;

use crate::util::approx_equal;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {r, g, b}
    }

    pub fn new_black() -> Self {
        Color { r: 0.0, g: 0.0, b: 0.0 }
    }

 
}


impl PartialEq for Color {   
    fn eq(&self, other: &Self) -> bool {
        approx_equal(self.r, other.r) &&
        approx_equal(self.g, other.g) &&
        approx_equal(self.b, other.b)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color::new(self.r + other.r, 
                   self.g + other.g, 
                   self.b + other.b)
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Color::new(self.r - other.r, 
                   self.g - other.g, 
                   self.b - other.b)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul<i32> for Color {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        let mul = rhs as f64;
        Color::new(self.r * mul, self.g * mul, self.b * mul)
    }
}

/* Hadamard product */
impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
         Color::new(self.r * other.r, 
                    self.g * other.g, 
                    self.b * other.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        
        assert_eq!(c * 2, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }

}
