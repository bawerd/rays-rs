use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Tuple {
    x: f32, 
    y: f32, 
    z: f32, 
    w: u8
}
pub type Point = Tuple;
pub type Vector = Tuple;

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

pub fn tuple(x: f32, y:f32, z:f32, w:u8 ) -> Tuple {
    Tuple { x, y, z, w }
}

pub fn point(x: f32, y:f32, z:f32) -> Point {
    Point { x, y, z, w: 1 }
}

pub fn vector(x: f32, y:f32, z:f32) -> Vector {
    Tuple { x, y, z, w: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_is_a_point() {
        /* A tuple with w=1 is a point */
        let a = tuple(4.3, -4.2, 3.1, 1); 
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1);
    }

    #[test]
    fn tuple_is_a_vector() {
        /* A tuple with w=0 is a vector */
        let a = tuple(4.3, -4.2, 3.1, 0); 
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0);
    }

     #[test]
    fn create_point() {
        /* Creates tuples with w=1 */
        let a = point(4.0, -4.0, 3.0); 
        let b = tuple(4.0, -4.0, 3.0, 1);
        assert_eq!(a.x, 4.0);
        assert_eq!(a.y, -4.0);
        assert_eq!(a.z, 3.0);
        assert_eq!(a.w, 1);
        assert_eq!(a, b);
    }

       #[test]
    fn create_vector() {
        /* Creates tuples with w=0 */
        let a = vector(4.0, -4.0, 3.0); 
        let b = tuple(4.0, -4.0, 3.0, 0);
        assert_eq!(a.x, 4.0);
        assert_eq!(a.y, -4.0);
        assert_eq!(a.z, 3.0);
        assert_eq!(a.w, 0);
        assert_eq!(a, b);
    }

    #[test]
    fn add_tuples() {
        /* Adding two tuples */
        let a = tuple(3.0, -2.0, 5.0, 1); 
        let b = tuple(-2.0, 3.0, 1.0, 0);
        assert_eq!(a + b, tuple(1.0, 1.0, 6.0, 1));
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
}