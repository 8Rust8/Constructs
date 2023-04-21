#![allow(dead_code, unused)]

use std::sync::Arc;

pub struct Person {
    pub name: Arc<String>,
}
pub mod arcc {
    use super::*;
    impl Person {
        pub fn new(name: Arc<String>) -> Person {
            Person { name }
        }
    }
}
