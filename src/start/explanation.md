# Explaining an embedded program

For our embedded programs we need the following attributes.

### `#![no_std]`

 The `#![no_std]` attribute indicates that the program will not make use of the standard library, the `std` crate. Instead it will use the `core` library, a subset of the standard library that does not depend on an underlying operating system (OS).

### `#![no_main]`

The `#![no_main]` attribute indicates that the program will use a custom entry point instead of the default `fn main() { .. }` one.

### `#[entry]`

The `#[entry]` attribute declares the custom entry point of the program. The entry point must be a divergent function whose return type is the never type `!`. The function is not allowed to return; therefore the program is not allowed to terminate.

To see what this does we can look into the `cortex-m` crate and have a
look at the [macro](https://github.com/rust-embedded/cortex-m-rt/blob/457a2e1820251ab7403d862e357431d879a627b7/src/lib.rs#L526)

The entry point of your program is usually the `main` function.

#### Lets jump to the next step and setup our first embedded program.
