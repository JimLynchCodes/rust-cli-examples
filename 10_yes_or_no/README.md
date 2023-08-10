# Yes or No
Create a cli tool that asks the user a yes or no question!

<br/>

## Backstory
Imagine you want to make a cli tool that can ask the user questions and stop, waiting for the user to enter some input.

Sometimes the user doesn't always want to pass in _everything_ as args when running the cli tool. Prompting the user for input is a key skill to have in your toolbag for creating polished cli tools with a nice user experience!

<br/>

## The Exercise
Create a cli tool that simply asks the user the question, "yes or no".

After the question is printed the console should hang and wait for a user input.

The question should end by displaying [y/N] with the N capitalized to signify that it is the default input if the user simply presses the enter key without typing any text.

The cli tool should read these inputs as a "yes": y, Y, yes, Yes, YES...

The cli tool should read these inputs as a "no": n, N, no, No, NO...

After the user submits a choice either "You said yes!" or "You say no..." should be printed to the console.

<br/>

## Tests

### -- Unit tests --

- Asker: check that "ask_yes_or_no" returns a properly built inqure Confirm object.

- Responder: call with true, expect to return true string. Another test, same but for false case.

- Main: unit test for main where you mock "ask_yes_or_no", expect "get_response_from_answer" to be called with mocked response from "ask_yes_or_no" and mock its own return value. expect response from "get_response_from_answer" to be printed, and expect to return Ok of unit.

### -- Integration Tests --

- Run the binary using Cargo_bin, send Enter, y, Y, yes, YES, yEs, yeS, Yes, YeS, etc- expect stdout to have yes message.

send variations of "no" and expect no message to be displayed.

<br/>

## Skills Practiced

- Prompting the user for input

- Simulating keystroke events in integration tests

<br/>
