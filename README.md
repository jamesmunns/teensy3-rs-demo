# Teensy with Rust

This is an example project that consumes the teensy3-rs crate.

# Usage:

* Fork/Clone this repo
* Install xargo
* install `arm-none-eabi-gcc`
* Run these commands to flash to the Teensy with blinky lights:
    * `xargo build --release`
    * `arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabi/release/teensy3-rs-demo target/hex`
    * `teensy-loader-cli -w -s --mcu=mk20dx256 target/hex`

# Thanks, Citiations

This code is nearly entirely thanks to these resources:

[PJRC's Teensyduino libraries](https://github.com/PaulStoffregen/cores) for the Teensy3, which are used as bindings.

[Simon's teensy3-clock repo](https://github.com/SimonSapin/teensy-clock) for the rust main, build scripts, bindgen knowledge, et. al.

[Servo's fork of bindgen](https://github.com/servo/rust-bindgen)

## Dependencies

* A somewhat current Nightly Build of Rust (currently tested on `rustc 1.13.0-nightly (6ffdda1ba 2016-09-14)`)
* [Japaric's Xargo Tool](https://github.com/japaric/xargo) - used to cross compile libcore
* A somewhat current arm-none-eabi-gcc toolchain.
    * 4.9.3 seems to work with a slight linker hack
    * 6.x.x seems to work without hacks.

Sorry for the lack of definite documents. More to come.

## License

MIT License, unless otherwise specified