#![allow(dead_code, unused)]

use std::sync::Mutex;
use std::time::Duration;

pub struct Person {
    pub name: Mutex<String>,
}
pub mod mutexx {
    use std::ops::Deref;

    use super::*;
    impl Person {
        pub fn new(name: String) -> Person {
            Person {
                name: Mutex::new(name),
            }
        }

        pub fn set_name(&self, name: String) {
            let mut t = self.name.lock().unwrap();
            std::thread::sleep(Duration::from_millis(10));
            *t = name;
        }
    }
}
