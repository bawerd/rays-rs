pub fn approx_equal(a: f64, b: f64) -> bool {
    (a - b).abs() <= std::f64::EPSILON
}

pub fn array_approx_equal(a: &[f64], b: &[f64]) -> bool {
    for (i,j) in a.iter().zip(b.iter()) {
        if (i - j).abs() > std::f64::EPSILON { return false } 
    }
    
    true
}