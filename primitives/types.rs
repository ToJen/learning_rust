fn main() {

    let logical: bool = true;

    let a_float: f64 = 1.0; // regular annotation
    let an_integer = 5i32;  // suffix annotation

    let default_float = 3.2; // f64 by default
    let default_int = 321; // i32 by default

    let mut inferred_type = 12;
    println!("{}", inferred_type);
    inferred_type = 1045914051i64;
    println!("{}", inferred_type);

    let mut mutable = 12; // `mut` variables can be changed
    mutable = 21; // i32
    println!("{}", mutable);

    // mutable = true; // type can't be changed
    // println!("{}", mutable);

    let mutable = true; // shadowing
    println!("{}", mutable);
}