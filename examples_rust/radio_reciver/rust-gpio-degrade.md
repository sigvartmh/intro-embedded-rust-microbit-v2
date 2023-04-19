# Usefulness of `degrade` on GPIO

A common idiom on embedded systems are to have physical pins grouped by ports, depending on the device there may be one or more ports each having a set of pins.

For Nordic's nRF devices the port and pin are expressed as two numerical values, e.g. Port 0, Pin 3 or P0.3 for short. Other vendors may express this differently.

As Rust allows us to associate and enforce functionality via the type system, the Hardware Abstraction Layer ([nrf-hal][nrf-hal]) is designed to make use of this to ensure that the hardware is in a sensible state.

Example: Such as only providing the "setting output values" to a pin configured as an output pin, reading pin state on an input-capable/configured pin etc.

The nrf-hal building blocks for GPIO are defined in [nrf-hal-common/src/gpio.rs][gpio]

```
<cut>
/// Disconnected pin in input mode (type state, reset value).
pub struct Disconnected;

/// Input mode (type state).
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state).
pub struct Floating;
<cut>
```

PhantomData is interesting, but let us ignore that for now.

If we wish to have a pin that is configured as a floating input, we start by some way or another get hold of the `pac::Peripherals` struct and then use methods to get the desired setup as shown below:

```
    /// We now have the pac::Peripherals
    let p = hal::pac::Peripherals::take().unwrap();
    // Get the object representing GPIO port 0
    let p0 = gpio::p0::Parts::new(p.P0);
    // Configure as a floating input pin
    let p0_09_input = p0.p0_09.into_floating_input();
 
```

Now `p0_09_input` is a struct of type `P0_09<Input<Floating>>`, which we can either have our editor show or do the classic "assign to incorrect type u32 and have compiler tell us"-trick:

```
  error[E0308]: mismatched types
  --> src/main.rs:21:32
   |
21 |         let p0_09_input: u32 = p0.p0_09.into_floating_input();
   |                          ---   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found struct `P0_09`
   |                          |
   |                          expected due to this
   |
   = note: expected type `u32`
            found struct `P0_09<Input<Floating>>`

```


Given that we can express a pin being configured in a specific mode (`Input`), in a specific state (`Floating`), and our pin consumers (usually other peripherals, like `UARTE`) enforces that specific pin mode and state, would that mean we also must provide specific initialization functions for every single pin? 

Thankfully not, as the `degrade()` method allows us to "relax" or "degrade" the specific pin type we have configured, making it become a pin of type `Pin<Input<Floating>>` instead.

Example of using the `degrade()` method on our pin:

```
error[E0308]: mismatched types
  --> src/main.rs:21:32
   |
21 |         let p0_09_input: u32 = p0.p0_09.into_floating_input().degrade();
   |                          ---   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found struct `Pin`
   |                          |
   |                          expected due to this
   |
   = note: expected type `u32`
            found struct `nrf52840_hal::gpio::Pin<Input<Floating>>`

```

This allows avoiding the issue of implementing functions for each and every possible configuration of specific pin, something different chip revisions and device families might make even harder to do by having different pins associated with the same functionality.

Instead, when designing HAL libraries it is sufficient to express the need for a `Pin<Input<Floating>>`.

Using `UARTE` as an example, from [uarte.rs][uarte]:

```
pub struct Pins {
    pub rxd: Pin<Input<Floating>>,
    pub txd: Pin<Output<PushPull>>,
    pub cts: Option<Pin<Input<Floating>>>,
    pub rts: Option<Pin<Output<PushPull>>>,
}

pub fn new(uarte: T, mut pins: Pins, parity: Parity, baudrate: Baudrate) -> Self {
  // Lots of setup code removed from here
}

```

We see that to create an `UARTE` we must construct the `Pins` struct, with two required pins (rxd, txd) and two `Option`-al pins (cts, rts), and also provide `Parity` and `Baudrate`.

An example how to create the `Pins` struct:

```
let uarte_pins = uarte::Pins {
    txd: p0.p0_06.into_push_pull_output(gpio::Level::High).degrade(),
    rxd: p0.p0_08.into_floating_input().degrade(),
    cts: Some(p0.p0_07.into_floating_input().degrade()),
    rts: Some(p0.p0_05.into_push_pull_output(gpio::Level::High).degrade()),
};

```

For the full example using UARTE, see [nrf-hal hello-world][helloworld] example.

[nrf-rf]: https://github.com/nrf-rs
[nrf-hal]: https://github.com/nrf-rs/nrf-hal/
[gpio]: https://github.com/nrf-rs/nrf-hal/blob/master/nrf-hal-common/src/gpio.rs
[uarte]: https://github.com/nrf-rs/nrf-hal/blob/master/nrf-hal-common/src/uarte.rs
[hellowordl]: https://github.com/nrf-rs/nrf-hal/blob/master/examples/hello-world/src/main.rs