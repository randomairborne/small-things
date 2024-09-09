use std::cmp::Ordering;
use std::ops::{Range, RangeInclusive};

fn main() {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut data: Vec<f64> = Vec::new();
    while stdin.read_line(&mut input).unwrap() != 0 {
        let current = input.trim().parse::<f64>().unwrap();
        data.push(current);
        input.clear();
    }
    let count = data.len() as f64;

    let total = data.iter().sum::<f64>();
    let mean = total / count;
    let variance = data.iter().map(|item| (item - mean).powi(2)).sum::<f64>() / count;
    let mad = data.iter().map(|item| (item - mean).abs()).sum::<f64>() / count;
    let stddev = variance.sqrt();
    let range = range(&data);

    println!("Total: {total}");
    println!("Item Count: {count}");
    println!("Mean: {mean}");
    println!("Variance: {variance}");
    println!("Mad: {mad}");
    println!("Stddev: {stddev}");
    println!("Range: {range:?}")
}

fn f64_max(a: f64, b: f64) -> f64 {
    match a.partial_cmp(&b).unwrap() {
        Ordering::Less => b,
        Ordering::Equal => a,
        Ordering::Greater => a,
    }
}

fn f64_min(a: f64, b: f64) -> f64 {
    match a.partial_cmp(&b).unwrap() {
        Ordering::Less => a,
        Ordering::Equal => a,
        Ordering::Greater => b,
    }
}

fn range(items: &[f64]) -> RangeInclusive<f64> {
    if items.is_empty() {
        panic!("range function requires non-zero number of items in its first arg");
    }
    let mut min = items[0];
    let mut max = items[0];
    for item in items.iter().copied() {
        max = f64_max(item, max);
        min = f64_min(item, min)
    }
    min..=max
}
