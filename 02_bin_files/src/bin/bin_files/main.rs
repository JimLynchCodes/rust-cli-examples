mod mod1;
use crate::mod1::fn1;
mod mod2;
use crate::mod2::fn2;

mod nested_mod {
    pub(super) mod mod3; // loads nested_mod/mod3.rs
}

use crate::nested_mod::mod3::fn3;

fn main() {
    let sum = fn1() + fn2() + fn3();

    println!("{}", sum);
}
