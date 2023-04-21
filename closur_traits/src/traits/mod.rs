pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug)]
pub enum Fruits {
    Mango,
    Banana,
    Peach,
    Apple,
    Guava,
    Orange,
    Grapes,
    Dummy,
}

pub mod closer_traits {
    #![allow(unused)]
    use super::DayOfWeek;
    use super::Fruits;

    // this function returns a closure which muts a Fruit and retuens back Fruit
    // should have taken some number in place of Fruits to make it convinient
    pub fn eat_fruit(input: DayOfWeek) -> impl FnMut(Fruits) -> Fruits {
        use DayOfWeek::*;
        use Fruits::*;
        match input {
            Monday => |mut x| {
                x = Mango;
                x
            },
            Tuesday => |mut x| {
                x = Banana;
                x
            },
            Wednesday => |mut x| {
                x = Peach;
                x
            },
            Thursday => |mut x| {
                x = Apple;
                x
            },
            Friday => |mut x| {
                x = Guava;
                x
            },
            Saturday => |mut x| {
                x = Orange;
                x
            },
            Sunday => |mut x| {
                x = Grapes;
                x
            },
        }
    }
}
