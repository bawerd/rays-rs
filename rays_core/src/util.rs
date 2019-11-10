pub fn approx_equal(a: f64, b: f64) -> bool {
    (a - b).abs() <= std::f64::EPSILON
}


pub fn array_approx_equal(a: &[f64], b: &[f64]) -> bool {
    const EPSILON:f64 = 0.00001;

    for (i,j) in a.iter().zip(b.iter()) {
        let diff = (i - j).abs();

        if diff > EPSILON { return false; } 
    }
    
    true
}