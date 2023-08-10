pub fn get_fibonacci_sequence(n: u128) -> Vec<u128> {
    let mut seq = vec![];

    if n <= 1 {
        return seq;
    }

    let mut prev = 0;
    let mut current = 1;

    for _ in 1..=n {
        let next = prev + current;
        prev = current;
        current = next;
        seq.push(current);
    }

    seq
}

#[cfg(test)]
mod get_fibonacci_sequence_tests {
    use super::get_fibonacci_sequence;


    #[test]
    fn zero() {
        let result = get_fibonacci_sequence(0);
        assert_eq!(result.len(), 0);
    }
    
    #[test]
    fn one() {
        let result = get_fibonacci_sequence(1);
        assert_eq!(result.len(), 0);
    }
    
    #[test]
    fn two() {
        let result = get_fibonacci_sequence(2);
        assert_eq!(result.len(), 2);
        assert_eq!(*result.get(0).unwrap(), 1);
        assert_eq!(*result.get(1).unwrap(), 2);
    }
    
    #[test]
    fn seven() {
        let result = get_fibonacci_sequence(7);
        assert_eq!(result.len(), 7);
        assert_eq!(*result.get(0).unwrap(), 1);
        assert_eq!(*result.get(1).unwrap(), 2);
        assert_eq!(*result.get(2).unwrap(), 3);
        assert_eq!(*result.get(3).unwrap(), 5);
        assert_eq!(*result.get(4).unwrap(), 8);
        assert_eq!(*result.get(5).unwrap(), 13);
        assert_eq!(*result.get(6).unwrap(), 21);
    }

    // #[test]
    // fn three() {

    // }

    // #[test]
    // fn four() {

    // }

    // #[test]
    // fn five() {

    // }

    // #[test]
    // fn six() {

    // }

    // #[test]
    // fn seven() {

    // }

}