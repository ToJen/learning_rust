type NanoSecond = u64;
type Inch = u64;

// #[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
    let ns: NanoSecond = 5 as u64_t;
    let i: Inch = 2 as u64_t;

    // type aliases don't provide added type safety
    println!("{} nanoseconds + {} inches = {} units?",
            ns, i, ns+i);
}