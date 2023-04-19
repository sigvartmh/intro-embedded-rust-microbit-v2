# Setting up the toolchain

This page contains the OS agnostic setup for a few tools we will use
during the workshop.

### Prerequisites

#### Linux(Ubuntu): 

```console
sudo apt install -y libusb-1.0-0-dev libftdi1-dev libudev-dev libssl-dev
```

#### Windows
[C++ Build Tools for Visual Studio 2019 is installed.](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
On Windows you can use [vcpkg](https://github.com/microsoft/vcpkg#quick-start-windows):

Dynamic linking 64-bit
```console
vcpkg install libftdi1:x64-windows libusb:x64-windows
set VCPKGRS_DYNAMIC=1
```

Static linking 64-bit
```console
vcpkg install libftdi1:x64-windows-static-md libusb:x64-windows-static-md
```

#### macOS
```console
brew install libftdi
```

### Rust Toolchain

Since we don't have time to go into how to set up your custom cross-compiling
toolchain we will use the path of least resistance which is using [rustup](https://rustup.rs). This will 

If you want to look a bit more into this on how to create your own
sysroot have a look at [xargo](https://github.com/japaric/xargo) and
what targets `rustc` supports by calling `rustc --print target-list`.

#### Cross compilation targets

Since we are going to use a nRF52833 which is a Cortex-M4F with hardware floating point
(ARMv7E-M architecture). We need to add the following target.

``` console
rustup target add thumbv7em-none-eabihf
```

### Cargo tools

``` console
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

### `cargo-generate`
We'll use this later to generate a project from a template. (Maybe)

``` console
cargo install cargo-generate
```

###  `cargo-embed`

Is a superset of a great tool called probe-rs(https://github.com/probe-rs/probe-rs).

``` console
cargo install cargo-embed
```

### Knurling Tools

There is a great consultancy firm based in Berlin who have create some
nice tooling for embedded Rust.

From this we will use what is called `probe-run` and `defmt`.

``` console
cargo install probe-run
```
