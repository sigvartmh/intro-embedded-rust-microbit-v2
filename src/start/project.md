# Configuring a project

First we just create our standard Rust starting point with `cargo init
hello_world`.

Then in the top of the file we need to add.

``` rust
#![no_std]
#![no_main]
```

Further more we know that we are programming a cortex-m processor so we
would have to include some run-time crates.


```console
cargo add cortex-m cortex-m-rt
```

Then we write our main function
```rust
#[entry]
fn main() -> ! {
}
```

Try to compile this and see what errors you get.

As mentioned in embedded we can't have a main function exit which is why
you'll have to add an infinite loop in the end of the main function.

```rust
loop { continue; };
```

If you try to compile after fixing this error the linker will scream at
you about missing a `memory.x` file. This file contains information on
where the RAM and FLASH is and their sizes. Create `memory.x` and add
the following.

```text
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
```

This information is taken from the datasheet of the nRF52833.

Since we are going to use `cargo embed` to flash the chip we have to
setup a `Embed.toml` file.

```toml
[default.probe]
# The protocol to be used for communicating with the target.
protocol = "Swd"

[default.flashing]
# Whether or not the target should be flashed.
enabled = true
# Whether or not the target should be halted after reset.
# DEPRECATED, moved to reset section
halt_afterwards = false
# Whether or not bytes erased but not rewritten with data from the ELF
# should be restored with their contents before erasing.
restore_unwritten_bytes = false
# The path where an SVG of the assembled flash layout should be written to.
# flash_layout_output_path = "out.svg"

[default.reset]
# Whether or not the target should be reset.
# When flashing is enabled as well, the target will be reset after flashing.
enabled = true
# Whether or not the target should be halted after reset.
halt_afterwards = false

[default.general]
# The chip name of the chip to be debugged.
chip = "nRF52833_xxAA"
# A list of chip descriptions to be loaded during runtime.
chip_descriptions = []
# The default log level to be used. Possible values are one of:
#   "OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE" 
log_level = "INFO"

[default.rtt]
# Whether or not an RTTUI should be opened after flashing.
# This is exclusive and cannot be used with GDB at the moment.
enabled = false 
# A list of channel associations to be displayed. If left empty, all channels are displayed.
channels = [
    # { up = 0, down = 0, name = "name", format = "String" }
]
# The duration in ms for which the logger should retry to attach to RTT.
timeout = 3000
# Whether timestamps in the RTTUI are enabled
show_timestamps = true
# Whether to save rtt history buffer on exit.
log_enabled = false
# Where to save rtt history buffer relative to manifest path.
log_path = "./logs"

[default.gdb]
# Whether or not a GDB server should be opened after flashing.
# This is exclusive and cannot be used with RTT at the moment.
enabled = false
# The connection string in host:port format wher the GDB server will open a socket.
# gdb_connection_string
```

This file has a lot of settings but let us leave it like this for now.

We also need to create a cargo config file in our project root. With the
following path `.cargo/config`.

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = 'arm-none-eabi-gdb'
rustflags = [
    "-C", "link-arg=-Tlink.x",
]
```

Now it tells us to add a panic handler. This function is what will be
called if your program panics. So lets add a simple panic handler.

```rust
use core::panic::PanicInfo;
use cortex_m::asm;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // abort instruction: triggers a HardFault exception which causes probe-run to exit
    asm::udf()
}
```

Now we can try to flash this using `cargo embed --target thumbv7em-none-eabihf`.

The result of this is a bit disappointing since we don't have any way of getting output from our program
yet so lets add that. 

What we will use is buffers in RAM to put some
data inside. This can be done using a debugger and [RTT](https://wiki.segger.com/RTT). An easy way to achieve this is using the `rtt-target` crate.

```console
cargo add rtt-target
```

This crate also requires something called critical-sections. This is
enabled by doing the following edit to the Cargo.toml

```toml
cortex-m = { version = "0.7.7", features = ["critical-section-single-core"]}
```

We also need to enable the RTT console on the host side(your computer)
this is done by editing the `Embed.toml` file.
```toml
[default.rtt]
# Whether or not an RTTUI should be opened after flashing.
# This is exclusive and cannot be used with GDB at the moment.
enabled = true
```

We then need to include the following modification to our code.
```rust
use rtt_target::{rprintln, rtt_init_print};
#[entry]
fn main() -> ! {
    // Initializes the buffer in RAM
    rtt_init_print!();
    // Puts data in the buffer
    rprintln!("Hello, world!");
    loop {continue;};
}
```

Now we can flash this using `cargo embed --target thumbv7em-none-eabihf` command again and a console should pop up displaying hello world.
All this is good and dandy but all we have done is manipulating some
memory in RAM and the hello world of embedded is producing a blinky
program.

**_NOTE:_** If you don't want to specify the target every time you build you can add
the following to your cargo config.

```toml
[build]
target = "thumbv7em-none-eabihf"
```
