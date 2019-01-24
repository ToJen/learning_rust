use List::*;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

// methods can be attached to an enum
impl List {

    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
    
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }

}

fn main() {

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(3);
    list = list.prepend(5);

    println!("list has {} elements", list.len());
    println!("{}", list.stringify());
}