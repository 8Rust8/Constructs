#![allow(unused, dead_code)]

use std::fmt::Display;

fn main() {
    let numbers = vec![2, 3, 4, 1, 2, 3];
    println!("Largest number is  {}", largetst(&numbers));
    let numbers = vec![2.0, 65.0, 4.0, 85.1];
    println!("Largest number is  {}", largetst(&numbers));
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("Largest char is  {}", largetst(&char_list));
}

fn largetst<T: PartialEq + PartialOrd + Display>(list: &[T]) -> &T {
    let mut largetst = &list[0];
    for num in list {
        if num > largetst {
            largetst = num;
        }
    }

    &largetst
}
