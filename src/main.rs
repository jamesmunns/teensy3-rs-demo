#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use teensy3::bindings;
use teensy3::serial::Serial;

#[no_mangle]
pub unsafe extern fn main() {
    // Blink Loop

    bindings::pinMode(13, bindings::OUTPUT as u8);
    bindings::digitalWrite(13, bindings::LOW as u8);
    let mut ser = Serial{};

    loop {
        // Show we are alive
        alive();

        // If the serial write fails, we will halt (no more alive blinks)
        hello(&ser).unwrap();

        // Don't spam the console
        bindings::delay(1000);
    }
}

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

/// Send a message over the USB Serial port
pub fn hello(ser: &Serial) -> Result<(),()> {
    let msg = "Hello Teensy Rusty World!\n\r";
    ser.write_bytes(msg.as_bytes())
}
