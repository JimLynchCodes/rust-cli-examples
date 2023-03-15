use mylib::number_manager::get_num;

#[test]
fn uses_real_ones_in_integration_test() {
    assert_eq!(get_num(), 1);
}