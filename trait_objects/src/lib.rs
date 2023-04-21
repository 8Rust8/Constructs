#![allow(unused)]

pub mod traitobjs;

use traitobjs::traitobjs::{Bus, Car, Vehicle};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_objects() {
        let c = Car::new("Maruti".to_string());
        let b = Bus::new("Tata".to_string());
        let vc = Vehicle::new(Box::new(c));
        let vb = Vehicle::new(Box::new(b));

        assert_eq!(vc.item.get_model(), String::from("Maruti"));
        assert_eq!(vb.item.get_model(), String::from("Tata"));
        // assert_eq!(vc., 4);
    }
}
