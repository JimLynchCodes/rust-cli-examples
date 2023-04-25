pub fn fn1() -> u8 {
    1
}

// Putting the tests in a different module prevents them being unnecessarily packaged into the final binary.
#[cfg(test)]
mod fn1_tests {

    use crate::mod1::fn1;

    #[test]
    fn returns_1() {
        assert_eq!(fn1(), 1);
    }
}
