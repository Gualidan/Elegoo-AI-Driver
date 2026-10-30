use arduino_hal::pac::TC1;
use embedded_hal::digital::InputPin;

/// Waits for the pin to be and measures the time that it stays `HIGH`
///
/// # Panics
/// This function might panic if there's a failure while reading the pin's state
pub fn echo_time<P: InputPin>(pin: &mut P, tc1: &TC1) -> Option<u32> {
    let t0 = tc1.tcnt1().read().bits();

    while pin
        .is_low()
        .expect("Should've been able to read the pin's state")
    {
        let t1 = tc1.tcnt1().read().bits();
        let elapsed = t1.wrapping_sub(t0);

        if elapsed > 20_000 {
            return None;
        }
    }

    let rise = tc1.tcnt1().read().bits();

    while pin
        .is_high()
        .expect("Should've been able to read the pin's state")
    {
        let t1 = tc1.tcnt1().read().bits();
        let elapsed = t1.wrapping_sub(rise);

        if elapsed > 60_000 {
            return None;
        }
    }

    let fall = tc1.tcnt1().read().bits();

    Some(fall.wrapping_sub(rise).into())
}
