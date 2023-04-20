fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod main_runs {

    use crate::main;

    #[test]
    fn returns_nothing() {
        let result = main();
        assert!(result == ());
    }
}
