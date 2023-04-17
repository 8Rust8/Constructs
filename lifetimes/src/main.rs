#![allow(unused)]
use lifetimes::lifetimes::lifetimes::print_str;
use lifetimes::structs_lt::structs_lt::Hero;
//  project::crate::mod
fn main() {
    // str
    let st = "This is a staic str";
    let s = String::from("This is a string");
    let s2 = st.to_string(); // no issues as st is an static str , which means it has liefetime till the programm end
    print_str(&st);
    print_str(&s);
    print_str(&s2);

    // let a: Option<i32> = None;
    // let mut b: Option<String> = None;
    // b = a; //  it matters its None of which type

    //lets see structs lifetimes

    let heros = vec!["Xeyro", "Orb"];
    let mut hero1 = Hero::new(heros[0], 100);
    let mut hero2 = Hero::new(heros[1], 100);

    hero1.damage(10);
    hero1.damage(110);
}
