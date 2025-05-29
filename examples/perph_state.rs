#![no_std]
#![no_main]

/// GPIO interface
struct GpioConfig {
    periph: u8,
}

impl GpioConfig {
    pub fn set_enable(&mut self, is_enabled: bool) {
        // self.periph.
    }
}
//Zero Sized Types
