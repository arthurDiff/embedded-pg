#![no_std]
#![no_main]

use cortex_m::peripheral::{syst, Peripherals};
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> !{
    let peripherals = Peripherals::take().unwrap();
    let mut systick= peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(1_000);
    systick.clear_current();
    systick.enable_counter();
    while !systick.has_wrapped(){
        // unwrap ticks
    }
    loop{}
}