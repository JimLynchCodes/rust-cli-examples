/*
   This is another example of a cli tool that returns "Hello, World!"

   In this project we are overriding the implementation of the `println!` macro to return the stringified arguments, but only for the unit tests!

   Then we assert that main returns what the println! macro returns.

   _Note: The "run" command does actually work with this project since it expects main to return a unit, not a String! In a real world situation you would probably want to move the things you want to test into a separate function other than main!_

   
*/

pub mod hello_world_with_unit_test {

    pub static HELLO: &'static str = "Hello, world!";

    #[cfg(test)]
    macro_rules! println {
        ($val:expr) => {
            format!("{}", $val)
        };
    }

    pub fn main() -> String {
        println!(HELLO)
    }
}

#[cfg(test)]
mod tests {

    use crate::hello_world_with_unit_test::{main, HELLO};

    #[test]
    fn it_calls_println_with_hello_world() {
        assert_eq!(main(), HELLO);
    }
}
