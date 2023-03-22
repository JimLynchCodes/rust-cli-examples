mod asker;
mod responder;
use asker::ask_are_you_cool;
use responder::get_response_from_are_you_cool_answer;

fn main() {
    let ans = ask_are_you_cool();

    let res = get_response_from_are_you_cool_answer(ans);

    println!("{}", res);
}