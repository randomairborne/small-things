
pub fn find_a(principal: f64, rate: f64, time: i64) -> f64 {
    principal * ((rate + 100)/100).powi(time)
}