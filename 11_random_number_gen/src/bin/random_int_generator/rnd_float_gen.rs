use rand::thread_rng;
use rand::Rng;

use crate::constants::Bounds;

#[cfg(not(test))]
// generates a random f64 float between 0 and 1
pub fn generate_random_float(bounds: Bounds) -> f64 {
    match bounds {
        Bounds::IncludeBoth => thread_rng().gen_range(0.0..=1.0),
        Bounds::ExcludeBoth => thread_rng().gen_range(0.0000000001..1.0),
        Bounds::IncludeLeftExcludeRight => thread_rng().gen_range(0.0..1.0),
        Bounds::ExcludeLeftIncludeRight => thread_rng().gen_range(0.0000000001..=1.0),
    }
}

#[cfg(test)]
/// instead of actually generating a random num in unit tests, return 1 minus super small amount
pub fn generate_random_float(_bounds: Bounds) -> f64 {
    1.0 - 0.0000000001
}

#[cfg(test)]
mod rnd_float_gen_tests {

    use more_asserts::{assert_gt, assert_lt};

    use crate::constants::Bounds;
    use crate::rnd_float_gen::generate_random_float;

    #[test]
    /// asserting the result is greater than and less than the proper bounds
    fn generanting_random_ints_gt_lt() -> Result<(), Box<dyn std::error::Error>> {
        let lower_bound = 0.0;
        let upper_bound = 1.0;
        let increment = 0.0000000001;

        assert_gt!(
            generate_random_float(Bounds::IncludeBoth),
            lower_bound - increment
        );

        assert_lt!(
            generate_random_float(Bounds::IncludeBoth),
            upper_bound + increment
        ); // less than max+1

        assert_gt!(generate_random_float(Bounds::ExcludeBoth), lower_bound); // gt than 0

        assert_lt!(generate_random_float(Bounds::ExcludeBoth), upper_bound); // less than max

        assert_gt!(
            generate_random_float(Bounds::IncludeLeftExcludeRight),
            lower_bound - increment
        );

        assert_lt!(
            generate_random_float(Bounds::IncludeLeftExcludeRight),
            upper_bound
        );

        assert_gt!(
            generate_random_float(Bounds::ExcludeLeftIncludeRight),
            lower_bound
        );

        assert_lt!(
            generate_random_float(Bounds::ExcludeLeftIncludeRight),
            upper_bound + increment
        );

        Ok(())
    }
}
