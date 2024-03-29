mod asker;
mod responder;
use std::error::Error;

use asker::ask_yes_or_no;
use responder::get_response_from_answer;

fn main() -> Result<(), Box<dyn Error>> {
    let ans = ask_yes_or_no()?;

    let res = get_response_from_answer(ans);

    println!("{}", res);

    Ok(())
}
