
#![allow(unused)]
use std::rc::Rc;

#[derive(Debug)]
pub struct City {
    pub name: String,
    pub population: u32,
    pub city_history: Rc<String>,
}

impl City {
    pub fn new(name: String, population: u32, city_history: Rc<String>) -> Self { Self { name, population, city_history } }
}

#[derive(Debug)]
pub struct CityData {
    names: Vec<String>,
    histories: Vec<Rc<String>>,
}

impl CityData {
    pub fn new(names: Vec<String>, histories: Vec<Rc<String>>) -> Self { Self { names, histories } }
}


