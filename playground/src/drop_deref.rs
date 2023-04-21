use std::{fmt::Display, ops::Deref};

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Display for MyBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self)
    }
}
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Hello , i am groing out of scope ");
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    let mn = MyBox::new(String::from("Rust"));
    hello(&mn);
}
