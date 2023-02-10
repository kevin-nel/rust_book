#[allow(unused)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// absolute path
// use crate::front_of_house::hosting;
// relative path
// use self::front_of_house::hosting;
// reexporting
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // because of the use statement
    hosting::add_to_waitlist();
}

fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}

/* new names with as
use std::fmt::Result;
use std::io::Result as IoResult;
fn function1() -> Result {}
fn function1() -> IoResult<()> {}
*/

//external packages
// need to add  to Cargo.toml
/*
[dependencies]
rand = "0.5.5"
*/

use rand::Rng;
fn randon_number() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}

// reexporting?

// When we bring a name into scope with the use keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use. This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.

// nested paths and globs
use std::collections::*;
use std::io::{self, Write};
