#![allow(dead_code)]

enum WebEvent {
    // enum could be unit-like
    PageLoad,
    PageUnload,
    // or like tuple structs
    KeyPress(char),
    Paste(String),
    // or like structures
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),

        // destructure `c` from inside the inum
        WebEvent::KeyPress(c) => println!("key '{}' was pressed", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),

        // destructure `Click` into `x` and `y`
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // to_owned makes a string slice an owned String
    let pasted = WebEvent::Paste("my sms".to_owned());
    let click = WebEvent::Click { x: 7, y: 6 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}