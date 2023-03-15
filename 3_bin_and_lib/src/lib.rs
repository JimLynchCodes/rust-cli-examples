/*
    Note how easy it is to write unit tests for this 

    public pure function now that we've extracted it out

    into this file... 😉

    Note that the Rust best practice is to put _unit tests_
    
    right in with the source code and only have _integration tests_

    in a /tests folder.
 */

pub mod num_functions {

    pub fn forty_two() -> u8 {
        42
    }
}

#[cfg(test)]
mod tests {

    use crate::num_functions::forty_two;

    #[test]
    fn it_works() {
        assert_eq!(forty_two(), 42);
    }
}
