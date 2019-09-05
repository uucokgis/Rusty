/*
This library was created following Packages, Crates and Modules section in rust e-book.


Importing nesting modules
example
use std::{cmd::Ordering, io};

another interesting example, instead of writing:
use std::io;
use std::io::Write;

use std::io::{self, Write};
choose this:

glob operator:
use std::collections::*;
*/


mod front_of_house;

pub fn eat_at_restaurant() {
    // Absolute path
    use crate::front_of_house::front_of_house::hosting;
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::front_of_house::serving::take_order();

    // use calling --relative
    use crate::front_of_house::front_of_house::serving;
    serving::take_order();
}

//Note that using self in this way might not be necessary in the future; it’s an inconsistency
// in the language that Rust developers are working to eliminate.

// using "use" calling  -- relative
//use self::front_of_house::serving;
pub fn eat_at_restaurantv2() {
//    serving::take_order(); // only this is public function in that trait
}
