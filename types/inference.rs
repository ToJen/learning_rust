fn main() {
    // the compiler knows elem is of type u8
    let elem = 5u8;

    let mut vec = Vec::new();
    // maintenmant, the compiler doesn't know the type of vec

    vec.push(elem);
    // now it knows elem is of type Vec<u8>

    println!("{:?}", vec)

}