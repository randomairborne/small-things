use rand::Rng;

fn main() {
    let chosen_nums = [gen(), gen(), gen(), gen(), gen(), gen()];
    let mut printed: Vec<(usize, usize)> = Vec::with_capacity(36);
    println!("numbers: {chosen_nums:?}");
    for (i1, num1) in chosen_nums.into_iter().enumerate() {
        for (i2, num2) in chosen_nums.into_iter().enumerate() {
            if i1 != i2 && num1.abs_diff(num2) % 5 == 0 && !printed.contains(&(num2, num1)) {
                printed.push((num1, num2));
                println!("num1: {num1} num2: {num2}");
            }
        }
    }
}

fn gen() -> usize {
    rand::thread_rng().gen_range(1..=2006)
}
