use arduino_hal::{
    Adc, Pins,
    port::{A0, A1, A2, D0, D1, D2, D5, D6, D7, D8, D9, D10, D11, D12, D13, Pin, mode},
};

pub type Serial =
    arduino_hal::Usart<arduino_hal::pac::USART0, Pin<mode::Input, D0>, Pin<mode::Output, D1>>;

pub type OutputPin<ID> = Pin<mode::Output, ID>;

pub type FloatingInputPin<ID> = Pin<mode::Input<mode::Floating>, ID>;
pub type AnalogInputPin<ID> = Pin<mode::Analog, ID>;

pub struct Board {
    pub serial: Serial,
    pub key_button: FloatingInputPin<D2>,
    pub motor_a_pwm: OutputPin<D5>,
    pub motor_b_pwm: OutputPin<D6>,
    pub motor_b_direction: OutputPin<D7>,
    pub motor_a_direction: OutputPin<D8>,
    pub ir_receiver: FloatingInputPin<D9>,
    pub servo_pan: OutputPin<D10>,
    pub servo_tilt: OutputPin<D11>,
    pub ultrasonic_echo: FloatingInputPin<D12>,
    pub ultrasonic_trigger: OutputPin<D13>,
    pub line_tracking_right: AnalogInputPin<A0>,
    pub line_tracking_middle: AnalogInputPin<A1>,
    pub line_tracking_left: AnalogInputPin<A2>,
}

impl Board {
    pub fn new(usart: arduino_hal::pac::USART0, pins: Pins, adc: &mut Adc) -> Self {
        let serial = arduino_hal::Usart::new(
            usart,
            pins.d0.into_floating_input(),
            pins.d1.into_output(),
            arduino_hal::hal::usart::BaudrateArduinoExt::into_baudrate(57600),
        );

        Self {
            serial,
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
