pub mod structs_lt {
    use colored::Colorize;

    pub struct Hero<'a> {
        name: &'a str,
        health: u32,
        is_alive: bool,
    }

    impl Hero<'_> {
        pub fn new(name: &'_ str, health: u32) -> Hero {
            Hero {
                name,
                health,
                is_alive: true,
            }
        }

        pub fn damage(&mut self, damage: u32) {
            if damage > self.health {
                self.health = 0;
            } else {
                self.health -= damage;
            }

            println!("{} recieved damage of {}", self.name, damage);
            println!("{}'s current health :: {}", self.name, self.health);
            if self.health == 0 {
                self.is_alive = false;
                println!("{} {} ", self.name, "lost!!!".color("red").bold());
            }
        }
    }
}
