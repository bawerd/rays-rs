use std::ops::{Index, Mul};
use std::cmp::PartialEq;

use crate::util::array_approx_equal;
use crate::tuples::*;


// Macro for implementing square matrices
macro_rules! mat_impl {
    ($mat_name:ident, $size:expr, $type:ident) => {

        #[derive(Debug, Clone)]
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
            pub fn new(matrix: [[$type; $size]; $size]) -> Self {
                Self {
                    size: $size,
                    m: matrix
                }
            }

            pub fn transpose(&self) -> Self {
                let mut m: [[$type; $size]; $size] =  [[0 as $type; $size]; $size];

                for (r, row) in self.m.iter().enumerate() {
                    for c in 0..m.len() {
                        m[c][r] = row[c];
                    }
                } 
               
                Self {
                    size: $size,
                    m: m
                }
            }

            #[allow(dead_code)]
            fn gen_submatrix(&self, remove_row: usize, remove_col: usize) -> [[$type; $size-1]; $size-1] {
                let mut m: [[$type; $size-1]; $size-1] =  [[0 as $type; $size-1]; $size-1];
                let mut r_i = 0;
                let mut c_i = 0;

                for (ri, row) in self.m.iter().enumerate() {
                        if ri == remove_row { continue; }
                        
                        for (ci, val) in row.iter().enumerate() {
                            if ci == remove_col { continue; }
                            m[r_i][c_i] = *val;
                            c_i += 1;
                        }
                    
                    r_i += 1;
                    c_i = 0;
                    }

                m
            }
        }       
    };
}




// create basic square matrices
mat_impl!(Matrix2x2, 2, f64); 
mat_impl!(Matrix3x3, 3, f64);
mat_impl!(Matrix4x4, 4, f64);


impl Matrix2x2 {
    pub fn determinant(&self) -> f64 {
        (self.m[0][0] * self.m[1][1]) - (self.m[0][1] * self.m[1][0])
    }
}

impl Matrix3x3 {
    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix2x2 {
        let m = self.gen_submatrix(remove_row, remove_col);
        Matrix2x2::new(m)
    }
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Self { size: 4, m: [[1., 0., 0., 0.],
                            [0., 1., 0., 0.],
                            [0., 0., 1., 0.],
                            [0., 0., 0., 1.]] }
    }

    pub fn submatrix(&self, remove_row: usize, remove_col: usize) -> Matrix3x3 {
       let m = self.gen_submatrix(remove_row, remove_col);
       Matrix3x3::new(m)
    }
}

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = [[0.; 4]; 4];
        
        for row in 0..m.len() {
            for col in 0..m.len() {
                m[row][col] = self.m[row][0] * rhs.m[0][col] +
                            self.m[row][1] * rhs.m[1][col] +
                            self.m[row][2] * rhs.m[2][col] +
                            self.m[row][3] * rhs.m[3][col];
            }
        }


       Matrix4x4 { size: 4, m: m }
    }
}

impl Mul<Tuple> for Matrix4x4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let mut t = tuple(0., 0., 0., 0.);

        // dot product of each row and the tuple as a one column matrix
        for (i, row) in self.m.iter().enumerate() {
            let val = row[0] * rhs.x + 
                    row[1] * rhs.y +
                    row[2] * rhs.z +
                    row[3] * rhs.w;
            
            match i {
                0 => t.x = val,
                1 => t.y = val,
                2 => t.z = val,
                3 => t.w = val,
                _ => ()
            }
        }
        t
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


   #[test]
    fn multiply_matrix_with_tuple() {
        let a = Matrix4x4::new([
            [1., 2., 3., 4.],
            [2., 4., 4., 2.],
            [8., 6., 4., 1.],
            [0., 0., 0., 1.]
        ]);   

        let b = tuple(1., 2., 3., 1.) ;  

        let expected_result = tuple(18., 24., 33., 1.);

        assert_eq!(a * b, expected_result);
    }

   #[test]
    fn multiply_with_identity_yields_same_matrix() {
        let a = Matrix4x4::new([
            [0., 1., 2., 4.],
            [1., 2., 4., 8.],
            [2., 4., 8., 16.],
            [4., 8., 16., 32.]
        ]);   

        let identity = Matrix4x4::identity();  

        assert_eq!(a.clone() * identity, a);
    }

   #[test]
    fn multiply_tuple_with_identity_yields_same_tuple() {
        let a = tuple(1., 2., 3., 4.);
        let identity = Matrix4x4::identity();  

        assert_eq!(identity * a.clone(), a);
    }


   #[test]
    fn transpose_matrix() {
        let a = Matrix4x4::new([
            [0., 9., 3., 0.],
            [9., 8., 0., 8.],
            [1., 8., 5., 3.],
            [0., 0., 5., 8.]
        ]);   

        let expected_result = Matrix4x4::new([
            [0., 9., 1., 0.],
            [9., 8., 8., 0.],
            [3., 0., 5., 5.],
            [0., 8., 3., 8.]
        ]);        

        assert_eq!(a.transpose(), expected_result);

        // The transpose of the identity, equals the identity
        assert_eq!(Matrix4x4::identity().transpose(), Matrix4x4::identity());
    }

    #[test]
    fn determinant_of_2x2_matrix() {
        let a = Matrix2x2::new([
            [1., 5.],
            [-3., 2.]
        ]);

        assert_eq!(a.determinant(), 17.);
    }

    #[test]
    fn submatrix_of_3x3_matrix() {
        let a = Matrix3x3::new([
            [1., 5., 0.],
            [-3., 2., 7.],
            [0., 6., -3.]
        ]);

        let b = Matrix2x2::new([
            [-3., 2.],
            [0., 6.]
        ]);

        assert_eq!(a.submatrix(0,2), b);
    }


    #[test]
    fn submatrix_of_4x4_matrix() {
        let a = Matrix4x4::new([
            [-6., 1., 1., 6.],
            [-8., 5., 8., 6.],
            [-1., 0., 8., 2.],
            [-7., 1., -1., 1.]
        ]);

        let b = Matrix3x3::new([
            [-6., 1., 6.],
            [-8., 8., 6.],
            [-7., -1., 1.]
        ]);

        assert_eq!(a.submatrix(2,1), b);
    }
}
