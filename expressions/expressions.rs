fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;

        // this expression will be assigned to y
        // since there's no semicolon

        x_cubed + x_squared + x 
    };

    let z = {
        // semicolon surpresses the expression and
        // `()` is assigned to z
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}