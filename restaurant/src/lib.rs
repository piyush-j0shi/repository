// Using nested paths to clean up Large "use" Lists
// use std::cmp::Ordering;
// use std::io

//Those above lines are same as
//use std::{cmp::Ordering, io};

//One more example
//use std::io;
//use std::io::Write;

//We can write like
//use std::io{self, Write};

//if we want to bring all public items we can use "The Global Operator"
//use std::collections::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
}
