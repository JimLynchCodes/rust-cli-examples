mod mod1;
use crate::mod1::fn1;
mod mod2;
use crate::mod2::fn2;

fn main() {
    let sum = fn1() + fn2();

    println!("{}", sum);
}
