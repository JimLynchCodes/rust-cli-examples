// This is the function we want to test
fn double(x: i32) -> i32 {
    x * 2
}

// This is the function we want to mock
fn expensive_operation(x: i32) -> i32 {
    // simulate a long computation
    std::thread::sleep(std::time::Duration::from_secs(2));
    x + 1
}

// Here's how we can use the `mockall` crate to mock the `expensive_operation` function
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;

    // Create a mock version of the `expensive_operation` function
    mock! {
        fn expensive_operation(x: i32) -> i32 {
            // Just return `x` as the result
            x
        }
    }

    // Test the `double` function with the mocked `expensive_operation`
    #[test]
    fn test_double_with_mock() {
        // Create a new instance of the mocked `expensive_operation`
        let mut mocked_expensive_operation = Mockexpensive_operation::new();

        // Set the expectations for the mocked function
        mocked_expensive_operation
            .expect_expensive_operation()
            .times(1)
            .returning(|x| x + 1); // return `x + 1` instead of the actual expensive computation

        // Call the `double` function with the mocked `expensive_operation`
        let result = double(3, &mut mocked_expensive_operation);

        // Check that the result is as expected
        assert_eq!(result, 8);
    }
}

// This is the actual `double` function that takes a mutable reference to the `expensive_operation` function
// fn double(x: i32, expensive_operation: &mut dyn FnMut(i32) -> i32) -> i32 {
//     let result1 = expensive_operation(x);
//     let result2 = expensive_operation(result1);
//     result2
// }