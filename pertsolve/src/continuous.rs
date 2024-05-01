use std::f64::consts::E;

// a = Pe^(rt)

/// Because multiplication is commutative, these are the same- we're dividing by one
/// component of the exponent to find the other
#[inline]
fn find_tr(amount: f64, principal: f64, rate_or_time: f64) -> f64 {
    (amount / principal).ln() / rate_or_time
}

pub fn find_t(amount: f64, principal: f64, rate: f64) -> f64 {
    find_tr(amount, principal, rate)
}

pub fn find_a(principal: f64, rate: f64, time: f64) -> f64 {
    principal * E.powf(rate * time)
}

pub fn find_r(amount: f64, principal: f64, time: f64) -> f64 {
    find_tr(amount, principal, time)
}

pub fn find_r_hl(half_life: f64) -> f64 {
    0.5f64.ln() / half_life
}
