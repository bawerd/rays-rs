use std::ops::{Add, Sub, Neg, Mul, Div };
use std::cmp::PartialEq;
use crate::util::approx_equal;

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64, 
    pub y: f64, 
    pub z: f64, 
    pub w: f64
}

#[allow(dead_code)]
impl Tuple {
    pub fn magnitude(&self) -> f64 {
        let mag = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        mag.sqrt()
    }

    pub fn normalize(&self) -> Vector {
        vector(self.x / self.magnitude(), self.y / self.magnitude(), self.z / self.magnitude())  
    }

    pub fn dot(&self, rhs: Vector) -> f64 {
        self.x * rhs.x +
        self.y * rhs.y +
        self.z * rhs.z +
        self.w * rhs.w 
    }

    pub fn cross(&self, rhs: Vector) -> Vector {
        vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x
        )
    }
}

pub type Point = Tuple;
pub type Vector = Tuple;

//impl Eq for Tuple {}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        tuple(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        tuple(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
    }
}

impl Neg for Tuple {
     type Output = Self;

    fn neg(self) -> Self {
      tuple(-self.x, -self.y, -self.z, -self.w)
    }   
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        tuple(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Mul<i32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        let mul = rhs as f64;
        tuple(self.x * mul, self.y * mul, self.z * mul, self.w * mul)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        tuple(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

impl Div<i32> for Tuple {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        let div = rhs as f64; 
        tuple(self.x / div, self.y / div, self.z / div, self.w / div)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, rhs: &Self) -> bool {
        approx_equal(self.x, rhs.x) && 
        approx_equal(self.y, rhs.y) && 
        approx_equal(self.z, rhs.z) && 
        approx_equal(self.w, rhs.w)
    }
}

pub fn tuple(x: f64, y:f64, z:f64, w:f64 ) -> Tuple {
    Tuple { x, y, z, w }
}

pub fn point(x: f64, y:f64, z:f64) -> Point {
    Point { x, y, z, w: 1.0 }
}

pub fn vector(x: f64, y:f64, z:f64) -> Vector {
    Tuple { x, y, z, w: 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    /* Compare floating point numbers */
    fn approx_equal(a: f64, b: f64) -> bool {
        (a - b).abs() <= std::f64::EPSILON
    }

    #[test]
    fn tuple_is_a_point() {
        /* A tuple with w=1 is a point */
        let a = tuple(4.3, -4.2, 3.1, 1.0); 
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
    }

    #[test]
    fn tuple_is_a_vector() {
        /* A tuple with w=0 is a vector */
        let a = tuple(4.3, -4.2, 3.1, 0.0); 
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
    }

     #[test]
    fn create_point() {
        /* Creates tuples with w=1 */
        let a = point(4.0, -4.0, 3.0); 
        let b = tuple(4.0, -4.0, 3.0, 1.0);
        assert_eq!(a.x, 4.0);
        assert_eq!(a.y, -4.0);
        assert_eq!(a.z, 3.0);
        assert_eq!(a.w, 1.0);
        assert_eq!(a, b);
    }

       #[test]
    fn create_vector() {
        /* Creates tuples with w=0 */
        let a = vector(4.0, -4.0, 3.0); 
        let b = tuple(4.0, -4.0, 3.0, 0.0);
        assert_eq!(a.x, 4.0);
        assert_eq!(a.y, -4.0);
        assert_eq!(a.z, 3.0);
        assert_eq!(a.w, 0.0);
        assert_eq!(a, b);
    }

    #[test]
    fn add_tuples() {
        /* Adding two tuples */
        let a = tuple(3.0, -2.0, 5.0, 1.0); 
        let b = tuple(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a + b, tuple(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtracting_two_points() {
        /* Subtracting two points */
        let p1 = point(3.0, 2.0, 1.0); 
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        /* Subtracting two points */
        let p1 = point(3.0, 2.0, 1.0); 
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(p1 - v, point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        /* Subtracting two points */
        let p1 = vector(3.0, 2.0, 1.0); 
        let p2 = vector(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        /* Subtracting two points */
        let zero = vector(0.0, 0.0, 0.0); 
        let v = vector(1.0, -2.0, 3.0);
        let sub = zero - v;
        assert_eq!(sub, vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negate_tuple() {
        /* Negating a tuple */
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, tuple(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        /* Multiplying a tuple by a scalar */
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, tuple(3.5, -7.0, 10.5, -14.0));
        /* also able to use integer scalar */
        assert_eq!(a * 3, tuple(3.0, -6.0, 9.0, -12.0));
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        /* Multiplying a tuple by a fraction */
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn divide_tuple_by_scalar() {
        /* Multiplying a tuple by a fraction */
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2, tuple(0.5, -1.0, 1.5, -2.0));
        assert_eq!(a / 2.0, tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn computes_magnitude_of_vector() {
        /* Computing the magnitude of vectors */
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);

        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());

        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalize_vector() {
        /* Normalizing vectors */
        let v = vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), vector(1.0, 0.0, 0.0));

        let v = vector(1.0, 2.0, 3.0);
        let div = 14.0_f64.sqrt();
        assert_eq!(v.normalize(), vector(1.0/div, 2.0/div, 3.0/div));
        assert!(approx_equal(v.normalize().magnitude(), 1.0));
    }

    #[test]
    fn compute_dot_product() {
        /* The dot product of two tuples */
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
    
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn compute_cross_product() {
        /* The cross product of two vectors */
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
    
        assert_eq!(a.cross(b), vector(-1.0, 2.0, -1.0));
    }
}