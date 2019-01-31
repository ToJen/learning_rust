#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        
        'inner: loop {
            println!("Entered the inner loop");
            // break; // break inner loop
            break 'outer; // break outer loop
        }
        println!("This would never be reached");
    }
    println!("Exited the outer loop");
}