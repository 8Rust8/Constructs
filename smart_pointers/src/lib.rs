#![allow(unused)]

pub mod arcc;
pub mod boxx;
pub mod celll;
pub mod mutexx;
pub mod rcc;

#[cfg(test)]
mod tests {
    use crate::arcc::Person as arcPerson;
    use crate::boxx::Person as bPerson;
    use crate::celll::Person as cPerson;
    use crate::mutexx::Person as mPersion;
    use crate::rcc::Person as rcPerson;

    #[test]
    fn box_test() {
        let b1 = bPerson::new(String::from("Hero"));
        assert_eq!(b1.name, "Hero".to_string());
    }

    #[test]
    fn cell_test() {
        let b1 = cPerson::new(String::from("Hero"));
        // Notice that b1 is not mutable here , but still we are able to mutate the internal elements.
        // Thats interior mutability
        b1.name.set("Zero".to_string());
        assert_eq!(b1.name.into_inner(), "Zero".to_string());
    }

    #[test]
    fn rc_test() {
        use std::rc::Rc;
        let name = Rc::new("Hero".to_string()); // 1st reference
        let p1 = rcPerson::new(name.clone()); // 2nd reference
        let p2 = rcPerson::new(name.clone()); // 3rd reference

        assert_eq!(Rc::strong_count(&name), 3);
        std::mem::drop(p2); // one ref decreased
        assert_eq!(Rc::strong_count(&name), 2);
        std::mem::drop(name); // second reference decreased
        assert_eq!(Rc::strong_count(&p1.name), 1); // Even if the origional name is dropped , its clone are alive
        assert_eq!(p1.name.to_string(), "Hero".to_string()); // value still there
                                                             // p1 will go out of scope when programm ends and then the memmory for string "Hero" will be released
    }

    #[test]
    fn arc_test() {
        use std::sync::Arc;
        let name = Arc::new("Hero".to_string()); // 1st reference

        let thread = std::thread::spawn(move || {
            let p1 = arcPerson::new(Arc::clone(&name)); // 2nd reference
            let p2 = arcPerson::new(Arc::clone(&name)); // 3rd reference
            (p1, p2)
        });

        let (b1, b2) = thread.join().unwrap();
        // name is already moved in the clouser, so now p1 and p2 are only referencing to "Hero"
        assert_eq!(Arc::strong_count(&b1.name), 2);
        std::mem::drop(b2); // one ref decreased
        assert_eq!(b1.name.to_string(), "Hero".to_string()); // value still there
                                                             // p1 will go out of scope when programm ends and then the memmory for string "Hero" will be released
    }

    #[test]
    // this is Arc . Mutex both
    fn mutex_test() {
        use std::rc::Rc;
        use std::sync::Arc;
        let p = Arc::new(mPersion::new("Zero".to_string()));
        let mut handles = vec![];
        let nvec = ["One", "two", "three", "four"];
        for ele in nvec {
            let n = p.clone();
            let handel = std::thread::spawn(move || {
                n.clone().set_name(ele.to_string());
            });
            handles.push(handel);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let mut t = p.name.lock().unwrap();

        // this might fail also , because it depends on which thread ends last
        // if third thread will end last , the value will be three
        assert_eq!(*t, "four".to_string());
    }
}
