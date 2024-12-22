mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn t() {
            child::hello();
        }

        pub mod child {
            use super::{add_to_waitlist, t};

            pub fn hello() {
                // Items in child modules can use the items in their ancestor modules
                add_to_waitlist();
                t();
            }
        }
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}