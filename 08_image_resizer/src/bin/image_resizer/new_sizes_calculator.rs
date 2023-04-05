pub use crate::StepDirection;

pub fn calculate_new_sizes(
    direction: StepDirection,
    steps: u32,
    step_size: f64,
    original_width: u32,
    original_height: u32,
) -> Vec<(f64, f64, String)> {
    (1..(steps + 1))
        .map(|x| match direction {
            StepDirection::Up => (
                original_width as f64 * step_size.powf(x as f64),
                original_height as f64 * step_size.powf(x as f64),
                (step_size.powf(x as f64)).to_string(),
            ),

            StepDirection::Down => (
                (original_width as f64 / step_size.powf(x as f64)),
                (original_height as f64 / step_size.powf(x as f64)),
                format!("1-{}", step_size.powf(x as f64)),
            ),
        })
        .collect()
}

#[cfg(test)]
pub mod size_calculator_tests {

    use crate::new_sizes_calculator::calculate_new_sizes;
    pub use crate::StepDirection;

    #[test]
    fn calculates_new_sizes_larger() {
        let results = calculate_new_sizes(StepDirection::Up, 3, 2.0, 100, 200);

        let expected = vec![
            (200.0, 400.0, "2"),
            (400.0, 800.0, "4"),
            (800.0, 1600.0, "8"),
        ];

        for (index, (width, height, multiplier_str)) in results.iter().enumerate() {
            assert_eq!(*width, expected[index].0);
            assert_eq!(*height, expected[index].1);
            assert_eq!(multiplier_str, expected[index].2);
        }
    }

    #[test]
    fn calculates_new_sizes_smaller() {
        let results = calculate_new_sizes(StepDirection::Down, 3, 2.0, 100, 200);

        let expected = vec![
            (50.0, 100.0, "1-2"),
            (25.0, 50.0, "1-4"),
            (12.5, 25.0, "1-8"),
        ];

        for (index, (width, height, multiplier_str)) in results.iter().enumerate() {
            assert_eq!(*width, expected[index].0);
            assert_eq!(*height, expected[index].1);
            assert_eq!(multiplier_str, expected[index].2);
        }
    }
}
