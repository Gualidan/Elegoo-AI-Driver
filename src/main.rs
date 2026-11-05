#![no_std]
#![no_main]

use crate::ultrasonic::echo::echo_time;
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

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut trigg = pins.d13.into_output();
    let mut echo = pins.d12.into_floating_input();

    loop {
        trigg.set_low();
        arduino_hal::delay_us(10);

        trigg.set_high();
        arduino_hal::delay_us(10);
        trigg.set_low();

        let echo = echo_time(&mut echo, &dp.TC1);

        if let Some(echo) = echo {
            let distance_cm = echo / 116;
            uwriteln!(&mut serial, "Found object at {} cm", distance_cm);
        } else {
            uwriteln!(&mut serial, "Nothing found, perhaps target is too far away");
        }

        arduino_hal::delay_ms(100);
    }
}
