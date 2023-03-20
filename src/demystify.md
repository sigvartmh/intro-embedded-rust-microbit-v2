# How does the whole system work?

Using `cargo-generate` to generate your project together with `cortex-m-rt` and `cortex-m` abstracts away a whole lot of complexity. Not knowing about this complexity is usually fine in 99% of the cases where you program embedded, but if you are going to write a library or want to experiment with a more complex layout or do other fancy things. You'll end up reaching into this deepend of doing embedded programming, some of this knowledge is architecture specific and other is just general knowledge of compilers and linkers.


## A look at what cortex-m-rt does

### What does cortex-m-rt provide?

It tries to provide a generalized Rust runtime environment targetting the ARM Cortex-M processor architecture.
