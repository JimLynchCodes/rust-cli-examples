/*
    Note how the "mock"
*/

pub mod number_manager {

    #[cfg(test)]
    use crate::mock_ones::one;
    #[cfg(not(test))]
    use crate::real_ones::one;

    pub fn get_num() -> u8 {
        one()
    }
}

pub mod real_ones {

    pub fn one() -> u8 {
        1
    }
}

pub mod mock_ones {

    pub fn one() -> u8 {
        99
    }
}

#[cfg(test)]
mod tests {

    use crate::number_manager::get_num;

    #[test]
    fn it_uses_mock_ones_for_unit_test() {
        assert_eq!(get_num(), 99);
    }
}
