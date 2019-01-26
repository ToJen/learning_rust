fn main() {

    let _immutable_binding = 1;
    let mut mutable_binding = 2;

    println!("before mutation: {}", mutable_binding);

    mutable_binding += 20;

    println!("After mutation: {}", mutable_binding);

    // _immutable_binding *= 0;
}