fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integerValue, booleanValue) = pair;

    (booleanValue, integerValue)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32, f32);

fn main() {
    
    let long_tuple = ('t', false, 1u8);
    println!("long_tuple first value: {}", long_tuple.0);
    println!("long_tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // But long Tuples cannot be 
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // TODO ^ Uncomment the above 2 lines to see the compiler error

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("reversed pair is {:?}", reverse(pair));

    let matrix = Matrix(1.2, 2.3, 3.4, 4.5, 5.6);
    println!("matrix: {:?}", matrix);
}