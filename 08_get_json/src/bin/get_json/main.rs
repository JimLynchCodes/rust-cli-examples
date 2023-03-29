mod dynamic_json_approach;
mod serde_struct_approach;
mod shared_constants;

fn main() {
    let dynamic_json = dynamic_json_approach::get_some_json().unwrap();

    println!("dynamic_json: {:#?}", dynamic_json);

    let struct_deserialised_json = serde_struct_approach::get_some_json().unwrap();

    println!("struct_deserialised_json: {:#?}", struct_deserialised_json);
}

#[test]
/// Notice how in this _unit test_ we use the #[cfg(test)] macros to use mock implementations.
/// Note how difficult it is to call useful assertions when calling main like this...
fn gets_and_prints_json() {
    main();
}
