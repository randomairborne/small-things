use std::io::Write;

fn main() {
    let c: f64 = 3.0 * (10.0f64.powi(8));
    let c_squared: f64 = c.powi(2);

    let time = get_float("time");
    let speed = get_float("speed");
    let tp = time / (1.0 - (speed.powi(2) / c_squared)).sqrt();
    let dt = tp - time;
    println!("{dt}");
}

fn get_float(msg: &str) -> f64 {
    eprint!("Enter the {msg}: ");
    std::io::stderr().flush().unwrap();
    let mut input = String::new();
    loop {
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to open stdin");
        if let Ok(data) = input.trim().parse::<f64>() {
            return data;
        } else {
            eprintln!("Enter a valid float.");
        }
    }
}
