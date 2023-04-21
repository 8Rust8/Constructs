#![allow(dead_code, unused)]

use std::cell::{Cell, RefCell, RefMut};

pub struct Person {
    pub name: Cell<String>,
}
pub mod celll {
    use super::*;
    impl Person {
        pub fn new(name: String) -> Person {
            Person {
                name: Cell::new(name),
            }
        }
    }
}
