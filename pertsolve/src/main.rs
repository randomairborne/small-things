use pertsolve::{continuous, discrete};
use std::io::Write;

fn main() {
    eprint!("Do you wish to compute continuously or discretely? ");
    std::io::stderr().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin");
    let input = input.trim();
    match input.to_ascii_uppercase().trim() {
        "C" | "CONTINUOUS" => continuous_compute(),
        "D" | "DISCRETE" => discrete_compute(),
        _ => {
            eprintln!("Invalid choice");
            return;
        }
    };
}

fn continuous_compute() {
    eprint!("Pick the A = Pe^(rt) value (or h) you wish to compute: ");
    std::io::stderr().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin");
    let input = input.trim();
    let output = match input.to_ascii_uppercase().trim() {
        "A" => continuous::find_a(get_float("principal"), get_float("rate"), get_float("time")),
        "T" => continuous::find_t(
            get_float("amount wanted"),
            get_float("principal"),
            get_float("rate"),
        ),
        "R" => continuous::find_r(
            get_float("amount wanted"),
            get_float("principal"),
            get_float("time"),
        ),
        "H" | "HL" | "HALF LIFE" => continuous::find_r_hl(get_float("half life")),
        _ => {
            eprintln!("Invalid choice");
            f64::NAN
        }
    };

    println!("{input} = {output}");
}

fn discrete_compute() {
    eprint!("Pick the A = (P * (1 + r)^t) - 1 value you wish to compute: ");
    std::io::stderr().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin");
    let input = input.trim();
    let output = match input.to_ascii_uppercase().trim() {
        "A" => continuous::find_a(get_float("principal"), get_float("rate"), get_float("time")),
        "T" => {
            let y_rate = get_float("rate");
            let c_per_rate = get_float("compounds per rate");
            let r_effective = discrete::effective_rate(y_rate, c_per_rate);
            continuous::find_t(
                get_float("amount wanted"),
                get_float("principal"),
                r_effective,
            )
        }
        "R" => continuous::find_r(
            get_float("amount wanted"),
            get_float("principal"),
            get_float("time"),
        ),
        "H" | "HL" | "HALF LIFE" => continuous::find_r_hl(get_float("half life")),
        _ => {
            eprintln!("Invalid choice");
            f64::NAN
        }
    };

    println!("{input} = {output}");
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
