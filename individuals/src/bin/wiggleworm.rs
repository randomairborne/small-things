use std::thread::sleep;
use std::time::Duration;
const TIME: Duration = Duration::from_millis(50);
fn main() {
    loop {
        sleep(TIME);
        println!("wiggle");
        sleep(TIME);
        println!(" wiggle");
        sleep(TIME);
        println!("  wiggle");
        sleep(TIME);
        println!("   wiggle");
        sleep(TIME);
        println!("   wiggle");
        sleep(TIME);
        println!("  wiggle");
        sleep(TIME);
        println!(" wiggle");
        sleep(TIME);
        println!("wiggle");
    }
}
