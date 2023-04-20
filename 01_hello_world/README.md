# Hello, World!
A barebones project that just prints the text "Hello, World"!

<br/>

## Backstory
Imagine you want to make the simplest Rust project possible- just the stuff needed to make a Rust project run.

The idea here is that if you can write a Rust project that compiles, runs, and does _something_ then you can build off this and build all the things you could ever imagine! 🌈

<br/>

## The Exercise
Write the simplest Rust project possible that prints some form of, "Hello, World!" to the console. 

<br/>

## Tests
The general pattern for where to put tests is Rust is that _unit tests_ should be in the same file it's function is defined, and _integration tests_ are put in the _tests_ folder next to the _src_ folder.

It's up to you to decide if / how you would unit test this, but it would be nice to at least have one integration test that asserts that the correct output was printed to the console.

<br/>

## Skills Practiced

- Installing and using `cargo`

- Creating a `main` function

- Creating an integration test

- Running a Rust project and tests

<br/>

## Commands

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

Run Mutation Test:
```bash
cargo mutants
```

Deploy to Cargo:
```bash
TODO...
```

Deploy to NPM:
```bash
TODO...
```
