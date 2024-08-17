use std::thread::sleep;
use std::time::Duration;
const TIME: Duration = Duration::from_millis(50);
const DISTANCE: usize = 10;
fn main() {
    let mut counter = 0;
    let mut inc = true;
    loop {
        sleep(TIME);
        println!("{:<counter$}wiggle", "");
        if inc {
            counter += 1;
        } else {
            counter -= 1;
        }
        if counter % DISTANCE == 0 {
            inc = !inc;
        }
    }
}
