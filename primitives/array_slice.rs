use std::mem;

fn analyse_slice(slice: &[i32]) {
    println!("first element of slice is {}", slice[0]);
    println!("slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1,2,3,4,5]; // type signature is superfluous
    let ys: [i32; 500] = [0; 500]; // fill up with the same values

    // 0-indexed
    println!("first element of the array: {}", xs[0]);

    println!("the array occupies {} bytes", mem::size_of_val(&xs));

    println!("'borrow' the whole array as a slice");
    analyse_slice(&xs);

    println!("'borrow' parts of the array as a slice");
    analyse_slice(&ys[10 .. 19]);

    // Out of bound indexing causes compile error
    // println!("{}", xs[5]);
}