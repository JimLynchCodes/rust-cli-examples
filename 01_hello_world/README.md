# Hello, World!
A barebones project that just prints the text "Hello, World"!

<br/>

## Backstory
Imagine you want to make the simplest Rust project possible- just the stuff needed to make a Rust project run.

The idea here is that if you can write a Rust project that compiles, runs, and does _something_ then you can build off this and create all the things you could ever imagine! 🌈

<br/>

## The Exercise
Write the simplest Rust project possible that prints some form of, "Hello, World!" to the console. 

<br/>

## Tests
Here I have one maybe pointless unit test just checking that the main function returns nothing.

I also have an integration test which verifies that the text "Hello, world!" is printed to the console and that the prompt appears on a new line.

I'm also adding some other interesting testing tools such as:

- Running unit tests with a code coverage output

- Running mutation tests to test the unit tests

- Running fuzz tests to check for other unexpected behavior

<br/>

## Skills Practiced

- Installing and using `cargo`

- Creating a `main` function

- Creating an integration test

- Running a Rust project and tests

<br/>

## Dev Commands

Run cli tool locally:
```bash
cargo run
```

Formatting / linting:
```bash
cargo fmt
cargo clippy
```

Run unit & integration tests:
```bash
cargo test
```

Run Units Tests Wih Code Coverage:
```bash
cargo tarpaulin
```

Run Tests Automatically On Code Change (Aka "Watch Mode"):
```bash
bacon test
```

Run Mutation Testing:
```bash
cargo mutants
```

Deploy to Cargo & NPM:

- Increment version in `Cargo.toml`
- Sign in with `cargo login` and `npm adduser`

Then deploy (with -n flag for a different name):
```bash
rust-to-npm-cli deploy -b -n @jimlynchcodes/hello-world-rs
```

To install via npm:
```bash
npm i -g @jimlynchcodes/hello-world-rs
```

Then run:
```bash
hello-world-rs
```
