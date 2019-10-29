use std::ops::Deref;

struct MyBox<T>(T); // tuple struct

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// requires a deref method that returns a reference to the inner data
impl<T> Deref for MyBox<T> {
    type Target = T; // define the associated type for `Deref` to use

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust")); // deref coercion in action
    hello(&m);
}
