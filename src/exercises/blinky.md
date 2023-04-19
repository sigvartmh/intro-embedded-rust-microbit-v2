# Blinky

For interfacing with the peripheral in the chip we need something more
advanced. This crate is called a PAC(Peripheral access crate).

This can be added using `cargo add nrf52833-pac` command.


Using this crate is done in the following way.

```rust
use nrf52833_pac as pac;

//inside a function
/* This is how to take the whole Peripheral structure */
let p = pac::Peripherals::take().unwrap(); //This can fail
```

From the presentation you can see that you can't take the peripheral
twice so you should probably try to use `Some`.

```rust
if let Some(p) = pac::Peripherals::take().unwrap(); //This can fail
```

To configure a pin as out put you can do the following:

```rust
p.P0.pin_cnf[28].write(|w| w.dir().output());
```

to set the output of a pin 

```rust
p.P0.out.write(|w| unsafe { w.bits((1 << 21)) });
```

This sets the 21 bit of the P0 register to 1 which also corresponds to
the Pin 21 on the device. Which tells the device to output a voltage on
Pin 21.

On the microbit we have a matrix of LED diodes where you'll have to pull
1 Pin Low while another Pin high so that the current flows through the
diode in the correct direction.

See the [board crate](https://github.com/nrf-rs/microbit/blob/3d4437902dc2a82571026f34e3da8dd71e81daab/microbit-common/src/v2/gpio.rs#L90) for their mapping.

To create a delay you can use the following function

```rust
for _ in 0..50_000 {
cortex_m::asm::nop();
}
```

This will just create 50 000 `nop` instructions 

For a solution you can look into `bare_metal` example.

This can be made a lot simpler by just using the microbit-v2 board crate
which includes the HAL(Hardware Abstraction Layer), PAC and Board
abstractions.

```rust
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::OutputPin};
use microbit::{board::Board, hal::timer::Timer};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

#[entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    //Take the timer structure to use the timer peripheral to create a
    // delay
    let mut timer = Timer::new(board.TIMER0);

    /* Sets the top controlling pin to low */
    let _ = board.display_pins.col1.set_low();
    let mut row1 = board.display_pins.row1;

    loop {
        /* Pulling the row 1 pin low to turn off the led */ 
        let _ = row1.set_low();
        timer.delay_ms(1_000_u16);
        /* Pulling the row 1 pin high to turn off the led */ 
        let _ = row1.set_high();
        timer.delay_ms(1_000_u16);
    }
}
```

The following code polls the button state of A and B try adding this to
your code.

```rust
let mut button_a = board.buttons.button_a;
//button b exists try to add this also
loop {
if let Ok(true) = button_a.is_high() {
//pressed
} else {
// Do nothing
}
}
```

