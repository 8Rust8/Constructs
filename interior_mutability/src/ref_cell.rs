pub mod ref_cell {
    #![allow(unused)]
    use std::{
        cell::RefCell,
        fmt::{self, Display},
    };

    pub struct Hero<'a> {
        name: &'a str,
        health: u32,
        is_alive: RefCell<bool>,
    }

    impl Hero<'_> {
        pub fn new(name: &'_ str, health: u32) -> Hero {
            let hero = Hero {
                name,
                health,
                is_alive: RefCell::new(true),
            };

            hero.set_life();
            hero
        }

        pub fn set_life(&self) {
            if self.health > 0 {
                self.is_alive.replace(true);
            } else {
                self.is_alive.replace_with(|_| false);
            }
        }
    }

    impl fmt::Debug for Hero<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Name :: {}, Health :: {} , Alive :: {}",
                self.name,
                self.health,
                self.is_alive.borrow()
            )
        }
    }
}
