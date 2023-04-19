# Logging

From now on we will try to use the `defmt` crate for logging to our RTT.
There are two dependencies you will need to add. 

```toml
defmt-rtt = "0.4"
defmt = "0.3.1"
```

The challenge for this exercise will be to print out a message when you
push a button.

To use `defmt` you will need the following changes to your cargo config
and remove the `rtt-target` crate as they are incompatible.

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = 'arm-none-eabi-gdb'
rustflags = [
    "-C", "link-arg=-Tlink.x",
    "-C", "link-arg=-Tdefmt.x",
    "-C", "link-arg=--nmagic",
]
```

Then you need to edit your `Embed.toml` file to use the Defmt logging
format in RTT.

```toml
[default.rtt]
# Whether or not an RTTUI should be opened after flashing.
# This is exclusive and cannot be used with GDB at the moment.
enabled = true
# A list of channel associations to be displayed. If left empty, all channels are displayed.
channels = [
    { up = 0, down = 0, name = "name", format = "Defmt" }
]
```

to enable RTT in your program you will only need to do.
```rust
use defmt_rtt as _
```
