# Arg Parse
Build a cli tool that reads and prints arguments!

<br/>

## Backstory
Imagine you want to build a cli tool. Well, if you want the user to be able to send some parameter or input as they run it... those are what we call _arguments._ 

<br/>

## The Exercise
Write a Rust program that takes two string arguments.

The first argument should be required (the program errors or panics if it is not supplied with `cargo run`).

The second argument is optional- if it is not supplied the program continues successfully, but if it is supplied then it is printed to the console.

If both arguments are passed with `cargo run` then both are printed to the console.

Additional arguments are ignored or (cause an error).

_Note: Don't worry about handling flags and fancier things just yet! In this exercise just handle the args as ___positional arguments___.

<br/>

## Tests
It's up to you to decide if / how you would unit test this.

There are a few integration tests you can write, where the test code more or less calls `cargo run`, passing in a different number of args for each test, and then expecting the proper text to be printed to the console or panicking to occur. 

<br/>

## Skills Practiced

- Reading positional cli arguments

- creating required and optional args

<br/>

## Bonus

Bonus points for finding multiple different ways of implementing this!
