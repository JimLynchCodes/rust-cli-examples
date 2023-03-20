/*
   This program can be run with the command: `cargo run`

   And tests can be run with the command: `cargo test

   Note how the "number_manager" loads the real one function

   for `cargo run` and a mocked one function for `cargo test`.
*/

extern crate mylib;

use mylib::number_manager::get_num;

fn main() {
    assert_eq!(get_num(), 1);
    println!("{}", get_num());
}
