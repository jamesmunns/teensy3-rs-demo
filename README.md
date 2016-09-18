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