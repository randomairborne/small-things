use rand::Rng;

fn main() {
    let mut options = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let first = pick_option(&mut options);
    options.push('0');
    let second = pick_option(&mut options);
    options.push(first);
    let third = pick_option(&mut options);

    let answer = [first, second, third];

    let stdin = std::io::stdin();
    let mut input = String::new();
    println!("I'm thinking of a number with three digits.");
    loop {
        stdin.read_line(&mut input).unwrap();
        let trimmed = input.trim();
        let guesses: Vec<char> = trimmed.chars().collect();
        input.clear();
        if guesses.len() != 3 || guesses.iter().any(|v| !v.is_ascii_digit()) {
            eprintln!("It's always three numbers.");
            continue;
        }
        if guesses == answer {
            println!("Correct! {}{}{} is my number.", first, second, third);
            break;
        }
        for (i, guess) in guesses.iter().enumerate() {
            let item = if answer[i] == *guess {
                "fermi"
            } else if answer.contains(guess) {
                "pico"
            } else {
                "bagel"
            };
            print!("{item} ")
        }
        println!();
    }
}

fn pick_option(data: &mut Vec<char>) -> char {
    let ones = rand::thread_rng().gen_range(0..data.len());
    data.swap_remove(ones)
}
