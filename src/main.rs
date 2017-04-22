#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use teensy3::bindings;
use teensy3::serial::Serial;
use teensy3::util::{
    digital_write,
    digital_read,
    pin_mode,
    delay,
    PinMode
};

fn read_analog(pin_id: u8) -> u8
{
    unimplemented!()
}

#[no_mangle]
pub extern fn main() {
    // Blink Loop

    pin_mode(13, PinMode::Output);
    digital_write(13, false);

    let mut ser = Serial{};

    loop {
        // Show we are alive
        alive();

        // If the serial write fails, we will halt (no more alive blinks)
        hello(&ser).unwrap();

        // Don't spam the console
        delay(1000);
    }
}

/// Blink the light twice to know we're alive
pub fn alive() {
    for _ in 0..2 {
        digital_write(13, false);
        delay(200);
        digital_write(13, true);
        delay(200);
        digital_write(13, false);
        delay(200);
    }
}

/// Send a message over the USB Serial port
pub fn hello(ser: &Serial) -> Result<(),()> {
    let msg = "Hello Teensy Rusty World!\n\r";
    ser.write_bytes(msg.as_bytes())
}
