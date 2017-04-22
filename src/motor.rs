use teensy3::util::{
    digital_write,
    analog_write,
    pin_mode,
    PinMode
};


/**
  Struct for controlling a SN754410 H bridge motor driver
*/
pub struct HBridge
{
    enable_pin: u8,
    input_pins: (u8, u8),
}

pub enum HBridgeState {
    Disable,
    Forward,
    Backward,
}


impl HBridge
{
    pub fn new(enable_pin: u8, input_pins: (u8, u8)) -> HBridge
    {
        pin_mode(enable_pin, PinMode::Output);
        pin_mode(input_pins.0, PinMode::Output);
        pin_mode(input_pins.1, PinMode::Output);

        HBridge {
            enable_pin: enable_pin,
            input_pins: input_pins,
        }
    }

    pub fn set_state(&self, state: HBridgeState, speed: u8)
    {
        match state
        {
            HBridgeState::Disable =>
            {
                digital_write(self.enable_pin, false);
            },
            HBridgeState::Forward =>
            {
                analog_write(self.enable_pin, speed);
                digital_write(self.input_pins.0, true);
                digital_write(self.input_pins.1, false);
            },
            HBridgeState::Backward =>
            {
                analog_write(self.enable_pin, speed);
                digital_write(self.enable_pin, true);
                digital_write(self.input_pins.0, false);
                digital_write(self.input_pins.1, true);
            }
        }
    }
}


