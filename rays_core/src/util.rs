pub fn approx_equal(a: f64, b: f64) -> bool {
    (a - b).abs() <= std::f64::EPSILON
}