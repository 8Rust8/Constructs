pub mod mutex {
    #![allow(unused)]
    use std::{
        cell::RefCell,
        fmt::{self, Display},
    };

    use std::sync::Mutex;

    pub struct Hero<'a> {
        name: &'a str,
        health: u32,
        is_alive: Mutex<bool>,
    }

    impl Hero<'_> {
        pub fn new(name: &'_ str, health: u32) -> Hero {
            let mut hero = Hero {
                name,
                health,
                is_alive: Mutex::new(true),
            };

            hero.set_life();
            hero
        }

        pub fn set_life(&mut self) {
            if self.health > 0 {
                *self.is_alive.get_mut().unwrap() = true;
                // other way is to create a mutex changer and then assign the value.  
                // but untill the mutex changer is not dropped, it wont allow anything to access the value
                //let mut my_mutex_changer = self.is_alive.lock().unwrap(); // check try_lock also
                //*my_mutex_changer = true;
                // std::mem::drop(my_mutex_changer); // or create the mutex changer inside a scope
            } else {
                *self.is_alive.get_mut().unwrap() = false;
            }
        }
    }

    impl fmt::Debug for Hero<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Name :: {}, Health :: {} , Alive :: {:?}",
                self.name, self.health, self.is_alive
            )
        }
    }
}
