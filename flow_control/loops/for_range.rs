fn main() {

    println!("style 0, exclusive range iterator");

    // goes from 1->100
    for n in 1..101 {
        if n % 15 == 0 {
            println!("6ixbuzz?");
        } else if n % 3 == 0 {
            println!("6ix {}", n);
        } else if n % 5 == 0 {
            println!("buzz {}", n);
        } else {
            println!("{}", n);
        }
    }

    println!("\nstyle 1, inclusive range iterator");

    for n in 1..=100 {
        if n % 15 == 0 {
            println!("6ixbuzz?");
        } else if n % 3 == 0 {
            println!("6ix {}", n);
        } else if n % 5 == 0 {
            println!("buzz {}", n);
        } else {
            println!("{}", n);
        }
    }
}