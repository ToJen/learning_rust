// globals declared before other scopes
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {} m8!!!", LANGUAGE);
    println!("Threshold is {}", THRESHOLD);
    println!("{} > {} is {}", n, THRESHOLD, is_big(n));
    println!("{} is {}er than {}", n, if is_big(n) { "great" } else { "less" }, THRESHOLD);
}