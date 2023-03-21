/*
   This program can be run with the command: `cargo run`

   And tests can be run with the command: `cargo test

   Note how the "number_manager" loads the real one function

   for `cargo run` and a mocked one function for `cargo test`.
*/

extern crate lib;

use lib::asker::ask_are_you_cool;
use lib::responder::get_response_from_are_you_cool_answer;

fn main() {

    let ans = ask_are_you_cool();

    let res = get_response_from_are_you_cool_answer(ans);

    println!("{}", res);
}
