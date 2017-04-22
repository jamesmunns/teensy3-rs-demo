#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use teensy3::serial::Serial;
use teensy3::util::{
    digital_write,
    analog_read,
    analog_write,
    pin_mode,
    delay,
    PinMode
};

mod motor;

#[no_mangle]
pub extern fn main() {
    // Blink Loop

    pin_mode(13, PinMode::Output);
    pin_mode(14, PinMode::Input);
    digital_write(13, true);

    let motor = motor::HBridge::new(20, (19, 18));

    let ser = Serial{};

    loop {
        motor.set_state(motor::HBridgeState::Forward, (analog_read(14) / 4) as u8);
        delay(10);
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
