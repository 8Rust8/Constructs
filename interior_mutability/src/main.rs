#![allow(unused)]
mod cell;
use std::rc::Rc;

//use std::fmt::{self, Debug};
//use cell::cell::Hero;
use interior_mutability::cell::cell as mod_cell;
use interior_mutability::rc::{City, CityData};
fn main() {
    let heros = vec!["Xeyro", "Orb"];
    let hero1 = mod_cell::Hero::new(heros[0], 0);
    //  hero1.set_life() is called from new only
    println!("{}", format!("{hero1:?}"));

    // Reference counter, rc

    let calgary = City::new(
        "Calgary".to_string(),
        1_200_000,
        Rc::new("Calgary began as a fort called Fort Calgary that...".to_string()),
    );
    // Pretend that this string is very very long

    let _canada_cities = CityData::new(vec![calgary.name], vec![calgary.city_history.clone()]);

    println!("Calgary's history is: {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary.city_history));
    let _new_owner = calgary.city_history.clone();
    dbg!(Rc::strong_count(&calgary.city_history));
    println!("New owner :: {}", _new_owner);
}
