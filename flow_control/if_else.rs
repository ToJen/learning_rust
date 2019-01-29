fn main() {
    let n = -12;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is 0", 0)
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // this expression returns i32
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // also returns an i32
            n / 2
        }; // don't forget semicolon

    println!("{} -> {}", n, big_n)
}