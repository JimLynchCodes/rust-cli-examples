
mod get_fibonacci_sequence;
use get_fibonacci_sequence::get_fibonacci_sequence;

fn main() {
    let seq = get_fibonacci_sequence(10);
    println!("{:?}", seq);
}
