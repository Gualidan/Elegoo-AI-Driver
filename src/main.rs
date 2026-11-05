#![no_std]
#![no_main]

use crate::{board::Board, ultrasonic::echo::echo_time};
use arduino_hal::{Adc, adc::AdcSettings};
use panic_halt as _;
use ufmt::uwriteln;

pub mod board;
pub mod ultrasonic;

/* Builds with: cargo build --target avr-none -C target-cpu=atmega328p -Z build-std=core
 * --target avr-none -C target-cpu= specifies AVR instruction encoding and correct linker
 */

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    dp.TC1.tccr1a().write(|w| unsafe { w.bits(0) });
    dp.TC1.tccr1b().write(|w| unsafe { w.bits(0b010) });

    let pins = arduino_hal::pins!(dp);
    let mut adc = Adc::new(dp.ADC, AdcSettings::default());
    let mut board = Board::new(dp.USART0, pins, &mut adc);

    loop {
        board.ultrasonic_trigger.set_low();
        arduino_hal::delay_us(10);

        board.ultrasonic_trigger.set_high();
        arduino_hal::delay_us(10);
        board.ultrasonic_trigger.set_low();

        let echo = echo_time(&mut board.ultrasonic_echo, &dp.TC1);

        if let Some(echo) = echo {
            let distance_cm = echo / 116;
            uwriteln!(&mut board.serial, "Found object at {} cm", distance_cm);
        } else {
            uwriteln!(
                &mut board.serial,
                "Nothing found, perhaps target is too far away"
            );
        }

        arduino_hal::delay_ms(100);
    }
}
