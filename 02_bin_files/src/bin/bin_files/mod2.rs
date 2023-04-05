pub fn fn2() -> u8 {
    2
}

// Note how this test still works without a module. 
#[test]
fn returns_2() {
    assert_eq!(fn2(), 2);
}
