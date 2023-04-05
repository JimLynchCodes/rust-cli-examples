pub fn fn3() -> u8 {
    3
}

#[cfg(test)]
mod fn3_tests {

    use crate::nested_mod::mod3::fn3;

    #[test]
    fn returns_3() {
        assert_eq!(fn3(), 3);
    }

}