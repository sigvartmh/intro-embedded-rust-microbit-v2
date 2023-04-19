# Display

Now we will try to use the Display abstraction included in the
`microbit` board crate.

To create a display you can do
```rust
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
let mut display = Display::new(board.display_pins);
```

Also have a look at the GrayScaleImage structure which could be
interesting for creating structures with different brightnesses.

```rust
    let b = inner_brightness;
    GreyscaleImage::new(&[
        [0, 7, 0, 7, 0],
        [7, b, 7, b, 7],
        [7, b, b, b, 7],
        [0, 7, b, 7, 0],
        [0, 0, 7, 0, 0],
    ])
```

Here are a link to more examples using the
[microbit](https://github.com/nrf-rs/microbit/tree/main/examples) board
crate.
