fn main() {
    // counter variable
    let mut n = 1;

    // loop while n < 101
    while n < 101 {
        if n % 15 == 0 {
            println!("6ixbuzz?");
        } else if n % 3 == 0 {
            println!("6ix {}", n);
        } else if n % 5 == 0 {
            println!("buzz {}", n);
        } else {
            println!("{}", n);
        }
        
        n += 1;
    }
}