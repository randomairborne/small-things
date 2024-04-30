use pertsolve::{find_a, find_r, find_r_hl, find_t};
use std::io::Write;

fn main() {
    eprint!("Pick the a = Pe^(rt) value (or h) you wish to compute: ");
    std::io::stderr().flush().unwrap();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read stdin");
    let input = input.trim();
    let output = match input.to_ascii_uppercase().trim() {
        "A" => find_a(get_float("principal"), get_float("rate"), get_float("time")),
        "T" => find_t(
            get_float("amount wanted"),
            get_float("principal"),
            get_float("rate"),
        ),
        "R" => find_r(
            get_float("amount wanted"),
            get_float("principal"),
            get_float("time"),
        ),
        "H" | "HL" | "HALF LIFE" => find_r_hl(get_float("half life")),
        _ => {
            eprintln!("Invalid choice");
            return;
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
