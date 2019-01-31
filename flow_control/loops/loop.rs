fn main() {
    let mut count = 0u32;

    println!("let's count to infinity and beyond");

    // infinte loop
    loop {
        count += 1;

        if count == 3 {
            println!("at three now!");

            // skip the rest of the iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("okay that's enough!");
            break;
        }
    }

}