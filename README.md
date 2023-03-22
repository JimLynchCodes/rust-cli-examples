# rust-cli-examples
Examples of clean and well-tested command line utilities, written in Rust. 🦀


## What is this repo?

This repo is a collection of many extremely small Rust projects.

This idea is to break down otherwise _scary_ Rust code into small, approachable little functions.

The numbers prefixing each folder are meant to be a recommended path to following in exploring these projects, but feel free to jump around as they are completely independent of each other! 

<br/>

## Running The Projects

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

In addition to just running the tests, you can check code coverage:
```
cargo tarpaulin --out Html
```

Then view the `tarpaulin-report.html` file in a browser.

<br/>

## Why Rust For CLI Tools?

There are a few reasons why Rust makes an excellent language for writing command line utilities.

- ### GC Free Programs 
    Often times there is a delay on startup and during heavy processing for languages with a garbage collector.
    
    Since Rust CLI programs do not have to boot up a vm or gc they can extremely fast, thus saving developer time, energy, and happiness in addition to company dollars!

- ### Cargo Is Awesome

    Cargo makes it very easy to build and deploy cli tools, and not just to cargo- you can also publish your super efficient binary to other package managers like npm!

- ### Very Small Projects
    CLI tools, like the ones here, can be very small and self contained. These projects are meant to be an approachable way to ease into nuanced syntax of Rust. Although building massively scalable and intricate servers and websites in Rust is awesome

    It can sometimes be challenging to deal with all the networking nuances that come with building, deploying, and maintaining these. Here we aim to write friendly Rust code that's _not_ scary! 

- ### Great For Low Level Things
    
    Rust code can interact extremely well with os calls, C/C++, and other low-level things that you might want a cli tool to do.

- ### Coding In Rust Is Fun!!
    
    Once you get the hang of it you may find that you actually really enjoy working on projects in Rust. There is a reason why it is voted the "Most Loved Programming Language" on the Stack Overflow survey every year! 

<br/>

## Contributing

Have a cool little cli tool you want to show off to others? Feel free to add a pull request! Also, feel free to open an issue just to discuss before hand or if you have any questions about the code / projects here. 

<br/>

## Thanks!

Dang, you really are still reading this document?? Well, I'd love to get your feedback so feel free to open an issue about whatever and/or message me on the socials @JimLynchCodes. Also, if you like this repo it would mean so much to me if you could please give it a star. ⭐️ Thanks for stopping by!
