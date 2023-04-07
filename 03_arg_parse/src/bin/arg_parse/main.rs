mod derive_style;
use crate::derive_style::read_args as read_args_derive_style;

mod builder_style;
use crate::builder_style::read_args as read_args_builder_style;

fn main() {
    let args_d = read_args_derive_style();
    println!(
        "arg1: {}, arg2: {:?}, flag: {:?}",
        args_d.arg1, args_d.arg2, args_d.flag
    );

    let args_b = read_args_builder_style();
    println!(
        "arg1: {}, arg2: {:?}, flag: {:?}",
        args_b.arg1, args_b.arg2, args_b.flag
    );
}
