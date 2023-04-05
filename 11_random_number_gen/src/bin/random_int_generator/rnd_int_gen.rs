use rand::thread_rng;
use rand::Rng;

use crate::constants::Bounds;

#[cfg(not(test))]
/// generates a random u32 unsigned integer between 0 and max_value
pub fn generate_random_int(max_value: u32, bounds: Bounds) -> u32 {
    match bounds {
        Bounds::IncludeBoth => thread_rng().gen_range(0..=max_value),
        Bounds::ExcludeBoth => thread_rng().gen_range(1..max_value),
        Bounds::IncludeLeftExcludeRight => thread_rng().gen_range(0..max_value),
        Bounds::ExcludeLeftIncludeRight => thread_rng().gen_range(1..=max_value),
    }
}

#[cfg(test)]
/// instead of actually generating a random num in unit tests, return max minus 1
pub fn generate_random_int(max_value: u32, bounds: Bounds) -> u32 {
    max_value - 1
}

#[cfg(test)]
mod rnd_int_gen_tests {

    use more_asserts::{assert_gt, assert_lt};

    use crate::constants::Bounds;
    use crate::rnd_int_gen::generate_random_int;

    #[test]
    /// asserting the result is greater than and less than the proper bounds
    fn generanting_random_ints_gt_lt() -> Result<(), Box<dyn std::error::Error>> {
        let lower_bound = 0;
        let upper_bound = 3;

        assert_gt!(
            generate_random_int(upper_bound, Bounds::IncludeBoth) as i32,
            (lower_bound as i32) - 1
        ); // gt than -

        assert_lt!(
            generate_random_int(upper_bound, Bounds::IncludeBoth),
            upper_bound + 1
        ); // less than max+1

        assert_gt!(
            generate_random_int(upper_bound, Bounds::ExcludeBoth),
            lower_bound
        ); // gt than 0

        assert_lt!(
            generate_random_int(upper_bound, Bounds::ExcludeBoth),
            upper_bound
        ); // less than max

        assert_gt!(
            generate_random_int(upper_bound, Bounds::IncludeLeftExcludeRight) as i32,
            (lower_bound as i32) - 1
        );

        assert_lt!(
            generate_random_int(upper_bound, Bounds::IncludeLeftExcludeRight),
            upper_bound
        );

        assert_gt!(
            generate_random_int(upper_bound, Bounds::ExcludeLeftIncludeRight),
            lower_bound
        );

        assert_lt!(
            generate_random_int(upper_bound, Bounds::ExcludeLeftIncludeRight),
            upper_bound + 1
        );

        Ok(())
    }

    #[test]
    /// asserting the result is equal to union of all valid possibilities
    fn generanting_random_ints_match() -> Result<(), Box<dyn std::error::Error>> {
        // let lower_bound = 0;
        let upper_bound = 2;

        match generate_random_int(upper_bound, Bounds::IncludeBoth) {
            0 | 1 | 2 => (),
            bad_num => panic!(
                "{}",
                format!(
                    "generated invalid int for Bounds::IncludeBoth -> {}",
                    bad_num
                )
            ),
        }

        match generate_random_int(upper_bound, Bounds::ExcludeBoth) {
            1 => (),
            bad_num => panic!(
                "{}",
                format!(
                    "generated invalid int for Bounds::ExcludeBoth -> {}",
                    bad_num
                )
            ),
        }

        match generate_random_int(upper_bound, Bounds::IncludeLeftExcludeRight) {
            0 | 1 => (),
            bad_num => panic!(
                "{}",
                format!(
                    "generated invalid int for Bounds::IncludeLeftExcludeRight -> {}",
                    bad_num
                )
            ),
        }

        match generate_random_int(upper_bound, Bounds::ExcludeLeftIncludeRight) {
            1 | 2 => (),
            bad_num => panic!(
                "{}",
                format!(
                    "generated invalid int for Bounds::ExcludeLeftIncludeRight -> {}",
                    bad_num
                )
            ),
        }

        Ok(())
    }
}
