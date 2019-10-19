use std::ops::{Index, Mul};
use std::cmp::PartialEq;

use crate::util::array_approx_equal;

// Macro for implementing square matrices
macro_rules! mat_impl {
    ($mat_name:ident, $size:expr, $type:ident) => {

        #[derive(Debug)]
        pub struct $mat_name {
            size: usize,
            m: [[$type; $size]; $size]
        }

        impl Index<usize> for $mat_name {
            type Output = [$type; $size];

            fn index(&self, idx: usize) -> &Self::Output {
                &self.m[idx] 
            }
        }

        impl PartialEq for $mat_name {
            fn eq(&self, other: &Self) -> bool {
                self.m.iter().zip(other.m.iter()).all(|(a,b)| array_approx_equal(a, b))
            }
        }

        impl $mat_name {
            pub fn new(matrix: [[$type; $size]; $size]) -> $mat_name {
                $mat_name {
                    size: $size,
                    m: matrix
                }
            }
        }       
    };
}

mat_impl!(Matrix4x4, 4, f64);
mat_impl!(Matrix3x3, 3, f64);
mat_impl!(Matrix2x2, 2, f64);

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = [[0.; 4]; 4];
        for row in 0..m.len() {
            for col in 0..len() {
                m[row][col] = self.m[row][0] * rhs.m[0][col] +
                            self.m[row][1] * rhs.m[1][col] +
                            self.m[row][2] * rhs.m[2][col] +
                            self.m[row][3] * rhs.m[3][col];
            }
        }

        Matrix4x4 { size: 4, m: m }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_4x4_matrix() {
        let m = Matrix4x4::new([
            [1. , 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5]
        ]);        

        assert_eq!(1., m[0][0]);
        assert_eq!(4., m[0][3]);
        assert_eq!(5.5, m[1][0]);
        assert_eq!(7.5, m[1][2]);
        assert_eq!(11., m[2][2]);
        assert_eq!(13.5, m[3][0]);
        assert_eq!(15.5, m[3][2]);
    }

    #[test]
    fn construct_2x2_matrix() {
        let m = Matrix2x2::new([
            [-3., 5.],
            [1., -2.]
        ]);        

        assert_eq!(-3., m[0][0]);
        assert_eq!(5., m[0][1]);
        assert_eq!(1., m[1][0]);
        assert_eq!(-2., m[1][1]);
    }

    #[test]
    fn construct_3x3_matrix() {
        let m = Matrix3x3::new([
            [-3., 5., 0.],
            [1., -2., -7.],
            [0., 1., 1.]
        ]);        

        assert_eq!(-3., m[0][0]);
        assert_eq!(-2., m[1][1]);
        assert_eq!(1., m[2][2]);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix4x4::new([
            [1. , 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.]
        ]);   

        let b = Matrix4x4::new([
            [1. , 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.]
        ]);        

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix4x4::new([
            [1. , 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.]
        ]);   

        let b = Matrix4x4::new([
            [2. , 3., 4., 5.],
            [6., 7., 8., 9.],
            [8., 7., 6., 5.],
            [4., 3., 2., 1.]
        ]);        

        assert_ne!(a, b);
    }

    #[test]
    fn multiply_two_matrices() {
        let a = Matrix4x4::new([
            [1., 2., 3., 4.],
            [5., 6., 7., 8.],
            [9., 8., 7., 6.],
            [5., 4., 3., 2.]
        ]);   

        let b = Matrix4x4::new([
            [-2., 1., 2., 3.],
            [3., 2., 1., -1.],
            [4., 3., 6., 5.],
            [1., 2., 7., 8.]
        ]);        

        let expected_result = Matrix4x4::new([
            [20., 22., 50., 48.],
            [44., 54., 114., 108.],
            [40., 58., 110., 102.],
            [16., 26., 46., 42.]
        ]);        

        assert_eq!(a * b, expected_result);
    }
}
