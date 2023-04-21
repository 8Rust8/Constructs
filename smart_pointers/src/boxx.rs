#![allow(dead_code, unused)]

pub struct Person {
    pub name: String,
    pub father: Option<Box<Person>>,
}
pub mod boxx {
    use super::*;
    impl Person {
        pub fn new(name: String) -> Person {
            Person { name, father: None }
        }
    }
}
