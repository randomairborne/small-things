use num_format::{Locale, ToFormattedString};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;
use std::time::Instant;

fn main() {
    let mut primes: Vec<u128> = Vec::new();
    let largest_number_wanted_input_vec: Vec<String> = env::args().collect();
    if largest_number_wanted_input_vec.len() != 2 {
        eprintln!("No largest number wanted found. Usage: `primes.exe 1000`");
        process::exit(21)
    }
    let largest_number_wanted_input_string = &largest_number_wanted_input_vec[1];
    let largest_number_wanted: u128 = match largest_number_wanted_input_string.parse() {
        Err(err) => {
            eprintln!("Could not parse input as integer `{:?}`", err);
            process::exit(20)
        }
        Ok(result) => result,
    };
    let path = Path::new("primes.txt");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    primes.push(2);
    println!("Finding primes...");
    let before = Instant::now();
    let m: usize = ((largest_number_wanted - 2) / 2) as usize;
    let mut candidates: Vec<bool> = vec![true; m];
    for j in 1..m {
        for i in 1..j + 1 {
            let not_prime = i + j + 2 * i * j;
            if not_prime > m {
                break;
            }
            candidates[not_prime - 1] = false;
        }
    }

    // Put primes in list
    for (i, c) in candidates.iter().enumerate() {
        if *c {
            let prime = 1 + 2 * (i + 1) as u128;
            primes.push(prime);
        }
    }

    println!(
        "Total time to find {} primes: {:.2?}",
        primes.len().to_formatted_string(&Locale::en),
        before.elapsed()
    );
    println!("Finished finding primes, writing to file.");
    let joined: String = primes
        .iter()
        .map(|&prime| prime.to_string() + "\n")
        .collect();
    match file.write(joined.as_bytes()) {
        Err(why) => panic!("There was an error: `{:?}` writing to file!", why),
        Ok(file) => file,
    };
}
