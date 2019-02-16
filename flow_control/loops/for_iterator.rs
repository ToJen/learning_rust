fn main() {
    let names = vec!["A", "B", "C"];

    // iter
    for name in names.iter() {
        match name {
            &"C" => println!("Found {}", name),
            _    => println!("Hello {}", name),
        }
    }

    // into_iter
    for name in names.into_iter() {
        match name {
            "C" => println!("Found {}", name),
            _   => println!("Hello {}", name),
        }
    }


    // iter_mut
    let mut mut_names = vec!["D", "E", "F"];

    for mut_name in mut_names.iter_mut() {
        match mut_name {
            &mut"C" => println!("found {}", mut_name),
            _       => println!("Hello {}", mut_name),
        }
    }
}