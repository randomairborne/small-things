use rand::Rng;

const INITIAL: u64 = 1_000_000;

fn main() {
    let mut rng = rand::rng();
    let mut shortest = vec![0; 4096];
    let mut current = Vec::with_capacity(100);
    for attempt in 1..=1_000_000 {
        let mut now = INITIAL;
        loop {
            now = rng.random_range(1..=now);
            current.push(now);
            if now == 1 {
                break;
            };
        }
        if current.len() < shortest.len() {
            std::mem::swap(&mut shortest, &mut current);
        }
        if shortest.len() == 1 {
            println!("{attempt}");
            break;
        }
        current.clear();
    }
    println!("{shortest:?}");
}
