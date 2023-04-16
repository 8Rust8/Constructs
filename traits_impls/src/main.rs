#![allow(unused, dead_code)]
use std::fmt::format;

#[derive(Debug)]
enum Lights {
    Neon,
    Halogen,
    LED,
}

#[derive(Debug)]
enum SpeedLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug)]
enum CoolerType {
    Fan(SpeedLevel),
    AC(SpeedLevel),
    HandFan(SpeedLevel),
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
}

impl Cooler {
    fn new() -> Cooler {
        Cooler {
            power: 10,
            brand: "TATA".to_string(),
            catageory: CoolerType::AC(SpeedLevel::Medium),
        }
    }
}
impl Bulb {
    fn new() -> Bulb {
        Bulb {
            power: 10,
            brand: "Surya".to_string(),
            catageory: Lights::Halogen,
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
}

// taking input as traits

fn get_bill(item: &impl Operations, day_hours: u32) -> String {
    item.generate_bill(day_hours)
}

fn main() {
    let b = Bulb::new();
    let c = Cooler::new();

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
