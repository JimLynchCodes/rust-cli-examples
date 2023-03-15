/*
   Here we export a binary but also have our

   own library to put the main logic in.

   Note that `cargo new bin-and-lib --bin --lib`

   doesn't work, but the Cargo.toml can be edited by hand.
*/

extern crate mylib;

use mylib::num_functions::forty_two;

fn main() {
    assert_eq!(forty_two(), 42);
    println!("{}", forty_two())
}
