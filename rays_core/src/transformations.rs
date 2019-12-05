use crate::tuples::*;
use crate::matrices::*;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4x4 {
    let mut id = Matrix4x4::identity();

    id[0][3] = x;
    id[1][3] = y;
    id[2][3] = z;

    id
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4x4 {
    let mut id = Matrix4x4::identity();

    id[0][0] = x;
    id[1][1] = y;
    id[2][2] = z;

    id
}

pub fn rotation_x(r: f64) -> Matrix4x4 {
    let mut id = Matrix4x4::identity();

    id[1][1] = f64::cos(r);
    id[1][2] = -f64::sin(r);
    id[2][1] = f64::sin(r);
    id[2][2] = f64::cos(r);

    id
}

pub fn rotation_y(r: f64) -> Matrix4x4 {
    let mut id = Matrix4x4::identity();

    id[0][0] = f64::cos(r);
    id[0][2] = f64::sin(r);
    id[2][0] = -f64::sin(r);
    id[2][2] = f64::cos(r);

    id
}

pub fn rotation_z(r: f64) -> Matrix4x4 {
    let mut id = Matrix4x4::identity();

    id[0][0] = f64::cos(r);
    id[0][1] = -f64::sin(r);
    id[1][0] = f64::sin(r);
    id[1][1] = f64::cos(r);

    id
}

pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix4x4 {
    let mut id = Matrix4x4::identity();

    id[0][1] = x_y;
    id[0][2] = x_z;
    id[1][0] = y_x;
    id[1][2] = y_z;
    id[2][0] = z_x;
    id[2][1] = z_y;

    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplying_by_translation_matrix() {
        let t = translation(5., -3., 2.);
        let p = point(-3., 4., 5.);
        let t_p = point(2., 1., 7.);

        assert_eq!(t * p, t_p);
    }

   #[test]
    fn multiplying_by_inverse_translation_matrix() {
        let t = translation(5., -3., 2.);
        let p = point(-3., 4., 5.);
        let inv = t.inverse().unwrap();

        let inverse_p = point(-8., 7., 3.);

        assert_eq!(inv * p, inverse_p);
    }

   #[test]
    fn translation_does_not_affect_vectors() {
        let t = translation(5., -3., 2.);
        let v = vector(-3., 4., 5.);

        assert_eq!(t * v, v);
    }

   #[test]
    fn scaling_point() {
        let t = scaling(2., 3., 4.);
        let p = point(-4., 6., 8.);
        let scaled_p = point(-8., 18., 32.);

        assert_eq!(t * p, scaled_p);
    }


   #[test]
    fn scaling_vector() {
        let t = scaling(2., 3., 4.);
        let v = vector(-4., 6., 8.);
        let scaled_v = vector(-8., 18., 32.);

        assert_eq!(t * v, scaled_v);
    }

   #[test]
    fn multiply_by_inverse_matrix() {
        let t = scaling(2., 3., 4.);
        let inv = t.inverse().unwrap();
        let v = vector(-4., 6., 8.);

        let scaled_v = vector(-2., 2., 2.);

        assert_eq!(inv * v, scaled_v);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let t = scaling(-1., 1., 1.);
        let p = point(2., 3., 4.);
        let reflected_p = point(-2., 3., 4.);

        assert_eq!(t * p, reflected_p);
    }

    #[test]
    fn rotating_point_around_x_axis() {
        let p = point(0., 1., 0.);
        let half_quarter = rotation_x(std::f64::consts::PI / 4.);
        let full_quarter = rotation_x(std::f64::consts::PI / 2_f64);

        assert_eq!(half_quarter * p, point(0., 2_f64.sqrt()/2., 2_f64.sqrt()/2.));
        assert_eq!(full_quarter * p, point(0., 0., 1.))
    }

    #[test]
    fn inverse_rotating_point_around_x_axis() {
        let p = point(0., 1., 0.);
        let half_quarter = rotation_x(std::f64::consts::PI / 4.);
        let inv = half_quarter.inverse().unwrap();

        assert_eq!(inv * p, point(0., 2_f64.sqrt()/2., -2_f64.sqrt()/2.));
    }

    #[test]
    fn rotating_point_around_y_axis() {
        let p = point(0., 0., 1.);
        let half_quarter = rotation_y(std::f64::consts::PI / 4_f64);
        let full_quarter = rotation_y(std::f64::consts::PI / 2_f64);

        assert_eq!(half_quarter * p, point(2_f64.sqrt()/2., 0., 2_f64.sqrt()/2.));
        assert_eq!(full_quarter * p, point(1., 0., 0.))
    }

    #[test]
    fn rotating_point_around_z_axis() {
        let p = point(0., 1., 0.);
        let half_quarter = rotation_z(std::f64::consts::PI / 4.);
        let full_quarter = rotation_z(std::f64::consts::PI / 2.);

        assert_eq!(half_quarter * p, point(-2_f64.sqrt()/2., 2_f64.sqrt()/2., 0.));
        assert_eq!(full_quarter * p, point(-1., 0., 0.))
    }

    #[test]
    fn shearing_all_axes() {
        let p = point(2., 3., 4.);
        let transform = shearing(1., 0., 0., 0., 0., 0.);
        assert_eq!(transform * p, point(5., 3., 4.));

        let transform = shearing(0., 1., 0., 0., 0., 0.);
        assert_eq!(transform * p, point(6., 3., 4.));

        let transform = shearing(0., 0., 1., 0., 0., 0.);
        assert_eq!(transform * p, point(2., 5., 4.));

        let transform = shearing(0., 0., 0., 1., 0., 0.);
        assert_eq!(transform * p, point(2., 7., 4.));

        let transform = shearing(0., 0., 0., 0., 1., 0.);
        assert_eq!(transform * p, point(2., 3., 6.));

        let transform = shearing(0., 0., 0., 0., 0., 1.);
        assert_eq!(transform * p, point(2., 3., 7.));
    }
}