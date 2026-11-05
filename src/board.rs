use arduino_hal::{
    Adc, Pins,
    port::{A0, A1, A2, D0, D1, D2, D5, D6, D7, D8, D9, D10, D11, D12, D13, Pin, mode},
};

pub struct Board {
    pub serial_rx: Pin<mode::Input<mode::Floating>, D0>,
    pub serial_tx: Pin<mode::Output, D1>,
    pub key_button: Pin<mode::Input<mode::Floating>, D2>,
    pub motor_a_pwm: Pin<mode::Output, D5>,
    pub motor_b_pwm: Pin<mode::Output, D6>,
    pub motor_b_direction: Pin<mode::Output, D7>,
    pub motor_a_direction: Pin<mode::Output, D8>,
    pub ir_receiver: Pin<mode::Input<mode::Floating>, D9>,
    pub servo_pan: Pin<mode::Output, D10>,
    pub servo_tilt: Pin<mode::Output, D11>,
    pub ultrasonic_echo: Pin<mode::Input<mode::Floating>, D12>,
    pub ultrasonic_trigger: Pin<mode::Output, D13>,
    pub line_tracking_right: Pin<mode::Analog, A0>,
    pub line_tracking_middle: Pin<mode::Analog, A1>,
    pub line_tracking_left: Pin<mode::Analog, A2>,
}

impl Board {
    pub fn new(pins: Pins, adc: &mut Adc) -> Self {
        Self {
            serial_rx: pins.d0.into_floating_input(),
            serial_tx: pins.d1.into_output(),
            key_button: pins.d2.into_floating_input(),
            motor_a_pwm: pins.d5.into_output(),
            motor_b_pwm: pins.d6.into_output(),
            motor_b_direction: pins.d7.into_output(),
            motor_a_direction: pins.d8.into_output(),
            ir_receiver: pins.d9.into_floating_input(),
            servo_pan: pins.d10.into_output(),
            servo_tilt: pins.d11.into_output(),
            ultrasonic_echo: pins.d12.into_floating_input(),
            ultrasonic_trigger: pins.d13.into_output(),
            line_tracking_right: pins.a0.into_analog_input(adc),
            line_tracking_middle: pins.a1.into_analog_input(adc),
            line_tracking_left: pins.a2.into_analog_input(adc),
        }
    }
}
