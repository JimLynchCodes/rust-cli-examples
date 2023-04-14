mod asker;
mod responder;
use std::error::Error;

use asker::ask_are_you_cool;
use responder::get_response_from_answer;

fn main() -> Result<(), Box<dyn Error>> {
    let ans = ask_are_you_cool()?;

    let res = get_response_from_answer(ans);

    println!("{}", res);

    Ok(())
}