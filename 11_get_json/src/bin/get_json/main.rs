use std::error::Error;

mod dynamic_json_approach;
mod serde_struct_approach;
mod shared_constants;

fn main() -> Result<(), Box<dyn Error>> {
    let dynamic_json = dynamic_json_approach::get_some_json()?;

    println!("dynamic_json: {:#?}", dynamic_json);

    let struct_deserialised_json = serde_struct_approach::get_some_json()?;

    println!("struct_deserialised_json: {:#?}", struct_deserialised_json);

    Ok(())
}

#[test]
// #[ignore]
/// Notice how in this _unit test_ we use the #[cfg(test)] macros to use mock implementation
/// of the "get_some_json" function.
fn gets_and_prints_json_unit_test() -> Result<(), Box<dyn Error>> {
    Ok(main()?)
}
