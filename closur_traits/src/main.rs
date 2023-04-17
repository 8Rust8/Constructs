use closur_traits::traits::{closer_traits::eat_fruit, DayOfWeek, Fruits};

fn main() {
    let mo = DayOfWeek::Monday;
    let tu = DayOfWeek::Tuesday;
    let we = DayOfWeek::Wednesday;
    let th = DayOfWeek::Thursday;
    let fr = DayOfWeek::Friday;
    let sa = DayOfWeek::Saturday;
    let su = DayOfWeek::Sunday;

    let mut day1 = eat_fruit(mo);
    println!("Having {:?} today ", day1(Fruits::Dummy));
    let mut day2 = eat_fruit(tu);
    println!("Having {:?} today ", day2(Fruits::Dummy));
    let mut day3 = eat_fruit(we);
    println!("Having {:?} today ", day3(Fruits::Dummy));
    let mut day4 = eat_fruit(th);
    println!("Having {:?} today ", day4(Fruits::Dummy));
    let mut day5 = eat_fruit(fr);
    println!("Having {:?} today ", day5(Fruits::Dummy));
    let mut day6 = eat_fruit(sa);
    println!("Having {:?} today ", day6(Fruits::Dummy));
    let mut day7 = eat_fruit(su);
    println!("Having {:?} today ", day7(Fruits::Dummy));
}
