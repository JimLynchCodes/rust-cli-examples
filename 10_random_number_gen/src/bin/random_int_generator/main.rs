/**
    This is a cli tool that generates a random number between two bounds.

    The examples demonstrate how to make random integers and floats that include or exclude either bound.
*/
mod rnd_int_gen;
use rnd_int_gen::generate_random_int;

mod rnd_float_gen;
use rnd_float_gen::generate_random_float;

mod constants;
use crate::constants::Bounds;

fn main() {
    let rand_uint_1 = generate_random_int(2, Bounds::IncludeBoth);
    let rand_uint_2 = generate_random_int(2, Bounds::ExcludeBoth);
    let rand_uint_3 = generate_random_int(2, Bounds::IncludeLeftExcludeRight);
    let rand_uint_4 = generate_random_int(2, Bounds::ExcludeLeftIncludeRight);

    let rand_float_1 = generate_random_float(Bounds::IncludeBoth);
    let rand_float_2 = generate_random_float(Bounds::ExcludeBoth);
    let rand_float_3 = generate_random_float(Bounds::IncludeLeftExcludeRight);
    let rand_float_4 = generate_random_float(Bounds::ExcludeLeftIncludeRight);

    println!("\n - random integers -");
    println!("includes both: {}", rand_uint_1);
    println!("excludes both: {}", rand_uint_2);
    println!("include left, exclude right: {}", rand_uint_3);
    println!("exclude left, include right: {}", rand_uint_4);

    println!(" -----------------");
    println!("\n - random floats -");
    println!("includes both: {}", rand_float_1);
    println!("excludes both: {}", rand_float_2);
    println!("include left, exclude right: {}", rand_float_3);
    println!("exclude left, include right: {}\n", rand_float_4);
}
