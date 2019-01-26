// surpress warnings from casts whcih result in overflows

#![allow(overflowing_literals)]

fn main() {

    let decimal = 65.4321_f32;

    // no implicit conversion
    // let integer: u8 = decimal;

    // explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("casting: {} -> {} -> {}", decimal, integer, character);

    // 1000 already fits in a u16
    println!("1000 as u16 is {}", 1000 as u16);
    println!("1000 as u8 is {}", 1000 as u8);

    println!("-1 as u8 is {}", (-1i8) as u8);

    println!("1000 mod 256 is: {}", 1000 % 256);
    println!("128 as i16 is: {}", 128 as i16);
    println!("128 as i8 is: {}", 128 as i8);

    println!("1000 as a u8 is : {}", 1000 as u8);
    // the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}