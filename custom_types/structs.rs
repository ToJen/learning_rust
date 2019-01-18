#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

// unit struct
struct Nil;

// tuple struct
struct Pair(i32, i32);

// struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// structs can be used in other structs
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // print debug struct
    println!("{:?}", peter);

    // make a `Point` instance
    let point: Point = Point { x: 2.5, y: 23.4 };
    println!("meet me at coordinates ({}, {})", point.x, point.y);

    // make a new point using a struct update syntax
    let point2 = Point { x: 9.6, ..point };
    // / `new_point.y` will be the same as `point.y` because we used that field from `point`
    println!("second point ({}, {})", point2.x, point2.y);

    // destructuring
    let Point { x: the_x, y: the_y } = point;


    let rectangle = Rectangle {
        p1: Point { x: the_x, y: the_y},
        p2: point2
    };

    let _nil = Nil;
    let pair = Pair(1, 2);
    println!("pair has {:?} and {:?}", pair.0, pair.1);

    // destructure tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}