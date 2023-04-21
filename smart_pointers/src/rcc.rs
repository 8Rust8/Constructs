#![allow(dead_code, unused)]

use std::rc::Rc;

pub struct Person {
    pub name: Rc<String>,
}
pub mod rcc {
    use super::*;
    impl Person {
        pub fn new(name: Rc<String>) -> Person {
            Person { name }
        }
    }
}
