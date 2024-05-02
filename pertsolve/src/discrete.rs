pub fn find_a(principal: f64, rate: f64, time: f64) -> f64 {
    principal * (1.0 + rate).powf(time)
}

pub fn effective_rate(rate: f64, period: f64) -> f64 {
    (1.0 + (rate / period)).powf(period) - 1.0
}
