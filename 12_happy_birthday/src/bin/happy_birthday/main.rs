mod asker;
use crate::asker::ask_for_birthday;

mod responder;
use crate::responder::respond_to_birthday;

fn main() {
    
    let date_of_birth = ask_for_birthday().unwrap();

    let response = respond_to_birthday(date_of_birth);

    println!("{}", response);
}
