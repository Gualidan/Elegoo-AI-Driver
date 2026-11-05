use ufmt::uwriteln;

use crate::board::Serial;

pub struct Logger<'a> {
    pub serial: &'a mut Serial,
    pub uptime_ticks: u64,
}

impl<'a> Logger<'a> {
    pub fn new(serial: &'a mut Serial) -> Self {
        Self {
            serial,
            uptime_ticks: 0,
        }
    }

    pub fn ticks(&mut self) {
        self.uptime_ticks += 1;
    }

    pub fn log_distance(&mut self, distance: u32) {
        uwriteln!(
            self.serial,
            "[{}] Found target at distance: {}\r\n",
            self.uptime_ticks,
            distance
        );
    }

    pub fn log(&mut self, message: &str) {
        uwriteln!(self.serial, "[{}] {}\r\n", self.uptime_ticks, message);
    }

    pub fn log_version(&mut self, version: &str) {
        uwriteln!(
            self.serial,
            "[{}] Firmware Version: {}\r\n",
            self.uptime_ticks,
            version
        );
    }
}
