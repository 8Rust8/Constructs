pub mod traitobjs {
    use std::clone;

    pub trait Drive {
        fn get_model(&self) -> String;
    }
    pub struct Vehicle {
        pub item: Box<dyn Drive>,
    }

    impl Vehicle {
        pub fn new(item: Box<dyn Drive>) -> Self {
            Self { item }
        }
    }
    pub struct Car {
        model: String,
    }

    impl Car {
        pub fn new(model: String) -> Self {
            Self { model }
        }
    }

    pub struct Bus {
        model: String,
    }

    impl Bus {
        pub fn new(model: String) -> Self {
            Self { model }
        }
    }

    impl Drive for Car {
        fn get_model(&self) -> String {
            self.model.clone()
        }
    }

    impl Drive for Bus {
        fn get_model(&self) -> String {
            self.model.clone()
        }
    }
}
