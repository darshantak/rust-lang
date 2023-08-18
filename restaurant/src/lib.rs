mod front_of_house {
    mod hosting {
        fn add_to_waitlist(){

        }
        fn seat_at_table(){

        }
    }

    mod serving {
        fn take_order(){

        }
        fn serve_order(){

        }
        fn take_payment(){

        }
    }
}


mod front_of_house_new{
    pub mod hosting{
        pub fn add_to_waitlist(){

        }
    }
}

pub fn eat_at_restaurant(){
    //This is the absolute path
    crate::front_of_house_new::hosting::add_to_waitlist();

    //This is the relative path
    front_of_house_new::hosting::add_to_waitlist();
}

fn serve_order(){}

mod back_of_house{
    fn fix_incorrect_orders(){
        cook_order();
        super::serve_order()
    }
    fn cook_order(){}
}

//New implementations of the module
mod back_of_house_new{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peachers"), }
        }
    }
}

pub fn eat_at_restaurant_new() {
    let mut meal = back_of_house_new::Breakfast::summer("French Toast");  
    //even tho we made the struct as public we still have the fields as private
    meal.toast = String::from("Wheat");
} 

mod back_of_house_enum {
    pub enum Appetizer {
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant_enum(){
    let order1 = back_of_house_enum::Appetizer::Soup;
    let order2 = back_of_house_enum::Appetizer::Salad;
}

mod hosting_module {
    pub mod hosting{
        pub fn add_to_waitlist() {
            
        }
    }
}
// use crate::hosting_module::hosting;
//To create this path to relative path

use self::hosting_module::hosting;

pub fn eat_at_restaurant_hosting(){
    hosting::add_to_waitlist();

}