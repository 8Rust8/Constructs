#![allow(unused, dead_code)]
use std::{fmt::{format, Error}, error::Error};

#[derive(Debug)]
enum Lights {
    Neon(Rating),
    Halogen(Rating),
    LED(Rating),
}

#[derive(Debug, PartialEq)]
enum Rating {
    High,
    Medium,
    Low,
}

#[derive(Debug)]
enum CoolerType {
    Fan(Rating),
    AC(Rating),
    HandFan(Rating),
}

struct Cooler {
    power: u32,
    brand: String,
    catageory: CoolerType,
}

struct Bulb {
    power: u32,
    brand: String,
    catageory: Lights,
}

pub trait Operations {
    fn switch_on(&self);
    fn switch_off(&self, power: u32) {
        println!("Saving power {}", power);
    }
    fn generate_bill(&self, day_hours: u32) -> String;
    fn get_rating(&self) -> Rating;
}

impl Cooler {
    fn new(power: u32, brand: String, rating: Rating) -> Cooler {
        Cooler {
            power,
            brand,
            catageory: CoolerType::AC(rating),
        }
    }
}
impl Bulb {
    fn new(power: u32, brand: String, rating: Rating) -> Bulb {
        Bulb {
            power,
            brand,
            catageory: Lights::Halogen(rating),
        }
    }
}

impl Operations for Cooler {
    fn switch_on(&self) {
        println!("Switching on a Cooler, Brand :: {:?}", self.catageory);
    }

    fn generate_bill(&self, day_hours: u32) -> String {
        format!(
            "Bill:: Item: Cooler. Brand: {:?}. Power consumed: {}",
            self.catageory,
            day_hours * self.power
        )
    }

    fn get_rating(&self) -> Rating {
        let rating = match self.catageory {
            CoolerType::Fan(r) => r,
            CoolerType::AC(r) => r,
            CoolerType::HandFan(r) => r,
        };
        rating
    }
}

impl Operations for Bulb {
    fn switch_on(&self) {
        println!("Switching on a Bulb, Brand :: {:?}", self.catageory);
    }

    fn generate_bill(&self, day_hours: u32) -> String {
        format!(
            "Bill:: Item: Bulb. Brand: {:?}. Power consumed: {}",
            self.catageory,
            day_hours * self.power
        )
    }

    fn get_rating(&self) -> Rating {
        let rating = match self.catageory {
            Lights::Neon(r) => r,
            Lights::Halogen(r) => r,
            Lights::LED(r) => r,
        };
        rating
    }
}

// taking input as traits
//Below all are same
//fn get_bill(item: &impl Operations, day_hours: u32) -> String {   // this is more readable
//fn get_bill<T: Operations>(item: T , day_hours: u32) -> String {  // this is more convnient if more than one input has same trait bound
fn get_bill<T>(item: &T, day_hours: u32) -> String
// this is more usefull if more mutiple types have different trait bounds
where
    T: Operations,
{
    item.generate_bill(day_hours)
}

// return a type using trait bound

fn get_optimal(item1: &Bulb, item2: &Cooler) -> Result<impl Operations, String> {
    let r1 = item1.get_rating();
    let r2 = item2.get_rating();

    match r1 == r2 {
        true => "Both are same. Neither is option ".to_string(),
        false => {
            match r1 == Rating::High {
                true => Ok(item1),
                false => Ok(item2),
            }
        },
    }
}

fn main() {
    let b = Bulb::new(10, "Surya".to_string(), Rating::High);
    let c = Cooler::new(15, "Voltas".to_string(), Rating::High);

    b.switch_on();
    b.switch_off(b.power);
    println!("{}", b.generate_bill(30));

    c.switch_on();
    c.switch_off(c.power);
    println!("{}", c.generate_bill(50));

    // use trait as an input type
    println!("Using traits input type: {}", get_bill(&b, 30));
    println!("Using traits input type: {}", get_bill(&c, 50));
}
