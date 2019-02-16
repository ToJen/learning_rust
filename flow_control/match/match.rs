fn main() {
    let number = 16;

    println!("Info on {}", number);
    match number {
        // match a single value
        1 => println!("one"),
        // match several values
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        // an inclusive range
        13...19 => println!("a teen!"),
        // handle other cases
        _ => println!("nothing special about {}", number),
    }

    let boolean = true;
    // match can be an expression too
    let binary = match boolean {
        // "arms" of a match must cover all possible values
        true  => 1,
        false => 0,
    };

    println!("{} {}", number, binary);
}