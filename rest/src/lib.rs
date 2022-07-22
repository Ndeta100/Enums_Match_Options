// #[cfg(test)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn sewat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
fn serve_order() {}
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn fall(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: string::from("apples"),
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}
pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    //relative path
    front_of_house::hosting::add_to_waitlist();
}
pub fn tttt() {
    let order1 = back_of_house::Appetizer::Soup;
    let mut meal = back_of_house::Breakfast::fall("Beans");
    meal.toast = String::from("Mable rye");
    println!("I'd like {} toast pleas", meal.toast);
}
