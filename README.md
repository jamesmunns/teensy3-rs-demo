# Teensy with Rust

This is an example project that consumes the teensy3-rs crate. It is intended to be forked, or copied into a new project. It provideds the following important components:

* `.cargo/config`: Instructs Cargo how which target to build
* `thumb7em-none-eabi.json`: Target configuration items specific to the build process
* `Cargo.toml`: Specification of profile.(dev|release)
* `mk20dx256.ld`: Linker file specific to the Teensy 3.1/3.2 processors
* `src/main.rs`: A few necessary compiler flags

# Examples

## Safe Components

Items used from the `teensy3` crate directly can be used as safe rust code. In this function, notice how there is no `unsafe` marker:

```rust
extern crate teensy3;
use teensy3::serial::Serial;

// ...

/// Send a message over the USB Serial port
pub fn hello(ser: &Serial) -> Result<(),()> {
    let msg = "Hello Teensy Rusty World!\n\r";
    ser.write_bytes(msg.as_bytes())
}
```

Items used from the `teensy3::bindings` module are NOT marked as safe (because they are direct C++ code mappings). These require an `unsafe` mark at either the function or block level:

```rust
extern crate teensy3;
use teensy3::bindings;

// ...

/// Blink the light twice to know we're alive
pub unsafe fn alive() {
    for _ in 0..2 {
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(200);
        bindings::digitalWrite(13, bindings::HIGH as u8);
        bindings::delay(200);
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(200);
    }
}
```

# Usage:

* Fork/Clone this repo
* Install xargo
* Install `arm-none-eabi-gcc`, add to $PATH
* Run `make flash`

# Dependencies

* A somewhat current Nightly Build of Rust (currently tested on `rustc 1.13.0-nightly (6ffdda1ba 2016-09-14)`)
* [Japaric's Xargo Tool](https://github.com/japaric/xargo) - used to cross compile libcore
* A somewhat current arm-none-eabi-gcc toolchain.
    * 4.9.3 seems to work with a slight linker hack
    * 6.x.x seems to work without hacks.

# License

Rust contributions are licensed under the MIT License.

**Please Note:** ASM, C, C++, and Linker Components of the `teensy3-sys` crate (a dependency of the `teensy3` crate) contain components licensed under the MIT License, PJRC's modified MIT License, and the LGPL v2.1. Please refer to individual components for more details.
