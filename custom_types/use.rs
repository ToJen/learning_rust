#![allow(dead_code)] // hide warnings for unused code

enum Status {
    Rich,
    Poor
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use Status::{Rich, Poor};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("Les riches ont beaucoup d'argent!"),
        Poor => println!("Les pauvres n'ont pas d'argent.."),
    }

    match work {
        Civilian => println!("Les civils travaillent!"),
        Soldier => println!("Les soldants se battent!"),
    }
}