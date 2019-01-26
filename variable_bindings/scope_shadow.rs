fn main() {
    // ths variable lives in the main function
    let long_lived_binding = 1;

    // this block has a smaller scope than the main
    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        // this binding shadows the outer one
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    // println!("outer short {}", short_lived_binding);

    println!("outer long {}", long_lived_binding);

    let long_lived_binding = 'd';
    println!("this also shadows the outer long {}", long_lived_binding);
}