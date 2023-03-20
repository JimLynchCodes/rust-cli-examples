# Rust Cli Examples

Some Rust cli examples as code by [Jim](https:www.github.com/jimlynchcodes)!

<br/>

## What is this repo?

This repo is a collection of many extremely small Rust projects.

The numbers prefixing each folder are meant to be a recommended path to following in exploring these projects, but feel free to jump around as they are completely independent of each other! 

<br/>

## Running Projects

In order to encapsulate each little example as a separate project we have freely called `cargo new ___` to create many projects, each named in a way to describe the _new thing_ it introduces.

First ensure you have rust and cargo installed:


You can then navigate into each project folder and run the cli tool:
```
cargo run
```

_Note: look for tips and information for a better understanding in the rust source code files of each project_ 


You can also run the automated tests:
```
cargo test
```

_Note: This runs both unit tests and integration tests!_

<br/>

## Why Rust For CLI Tools?

There are a few reasons why Rust makes an excellent language for writing command line utilities.

- It does not have to boot up a vm or garbage collector which allows virtually Rust projects to execute faster than those written in other languages, thus saving developer time, energy, and happiness in addition to company money.

- Cargo makes it very easy to build and deploy cli tools, and not just to cargo- you can also publish your super efficient binary to other package managers like npm!

- CLI tools, like the ones here, can be very small and self contained, and many people have found that working through these small projects is an approachable way to ease into the somewhat difficult syntax of Rust. Although building massively scalable and intricate servers and websites in Rust is awesome, it can sometimes be challenging to deal with all the networking nuances that come with building, deploying, and maintaining these. Here we aim to write friendly Rust code that's _not_ scary! 

- Rust code can interact well with low level APIs and can do virtually anything you could ask from a cli tool.

<br/>

## Contributing

Have a cool little cli tool you want to show off to others? Feel free to add a pull request! Also, feel free to open an issue just to discuss before hand or if you have any questions about the code / projects here. Also, if you like this repo it would mean so much to me if you could please give it a star. ⭐️ Thanks for stopping by!

