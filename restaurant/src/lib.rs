mod front_of_house;


// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;

// use std::collections::HashMap;

use hosting::add_to_waitlist;
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    add_to_waitlist();
    hosting::add_to_waitlist();

}


mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    fmt::Result::Ok(())
}

fn function2() -> io::Result<()> {
    io::Result::Ok(())
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    fmt::Result::Ok(())
}

fn function4() -> IoResult<()> {
    io::Result::Ok(())
}

fn deliver_order() {
    use std::collections::HashMap;

    let mut a = HashMap::new();
    a.insert(String::from("Blue"), 10);
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn test() {}

mod level1 {
    pub fn test1() {
        super::test();

        level2::cook_order();
    }

    pub mod level2 {
        pub fn cook_order() {
            crate::level1::test1();
        }
    }
}

mod back_of_house2 {
    use crate::back_of_house2;

    pub struct Breakfast {
        season: &'static str,
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Self::factory("summer", toast, "strawberries")
        }

        pub fn winter(toast: &str) -> Breakfast {
            Self::factory("winter", toast, "apples")
        }

        fn factory(season: &'static str, toast: &str, seasonal_fruit: &str) -> Breakfast {
            Breakfast {
                season,
                toast: String::from(toast),
                seasonal_fruit: String::from(seasonal_fruit),
            }
        }

        pub fn print(&self) {
            println!("{} - I'd like {} toast please, with: {}.", self.season, self.toast, self.seasonal_fruit);
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house2::Breakfast::summer("Rye");
    println!("I'd like {} toast please", meal.toast);
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    meal.print();

    let mut meal = back_of_house2::Breakfast::winter("Apples");
    println!("I'd like {} toast please", meal.toast);
    meal.toast = String::from("Mehmedlija");
    println!("I'd like {} toast please", meal.toast);

    meal.print();

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house2::Appetizer::Soup;
    let order2 = back_of_house2::Appetizer::Salad;

    dbg!(&order1);
    dbg!(&order2);
}

use rand::Rng;

fn rand_er() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}", secret_number);
}

use std::{cmp::Ordering, io as IO};

use std::io::{self as Tito, Write};

use std::collections::*;

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
