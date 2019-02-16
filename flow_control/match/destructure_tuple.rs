fn main() {
    let pair = (10, -2);

    match pair {
        (0, y) => println!("x is 0 and y is {:?}", y),
        (x, 0) => println!("x is {:?} and y is 0", x),
        _      => println!("pair is {:?}", pair),
        // ^ anchor (`_`) means don't bind value to a variable
    }
}