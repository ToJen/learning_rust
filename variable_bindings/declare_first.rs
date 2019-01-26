fn main() {
    let a_binding; // declared variable

    {
        let x = 9;
        a_binding = x * x; // initialize the bindings
    }

    println!("a binding's value: {}", a_binding);

    let another_binding;
    // println!("another binding's value: {}", another_binding);

    another_binding = 1;
    println!("{}", another_binding)
}