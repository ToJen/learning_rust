fn main() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);
    // println!("(unsigned) 1 - 2 = {}", 1u32 - 2); // underflow

    println!("true AND FALSe is {}", true && false);
    
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);

    println!("One million is written as {}", 1_000_000u32);
}