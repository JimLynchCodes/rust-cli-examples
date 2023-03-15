/*
   This program can be run with the command: `cargo run`

   Note that it requires two arguments, a _pattern_ and a _path_.

   An example that runs cleanly: `cargo run -- foo .`
*/

mod read_args;

fn main() {
    let result = read_args::get();
    println!("{:#?}", result);
}
