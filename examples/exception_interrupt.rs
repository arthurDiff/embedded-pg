#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::{ peripheral::syst::SystClkSource};
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::{debug, hio::{self, HostStream}};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);

    syst.set_reload(12_000_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    loop{}
}

#[exception]
fn SysTick(){
    static mut COUNT:u32 = 0;
    static mut STDOUT: Option<HostStream> = None;

    *COUNT += 1;

    // Lazy initialization
    if STDOUT.is_none(){
        *STDOUT = hio::hstdout().ok(); 
    }


    if let Some(hstdout) = STDOUT{
        write!(hstdout, "{}\n", *COUNT).ok();
    }

    // IMPORTANT omit this `if` block if running on real hard ware or your debugger will end in an inconsistent state
    if *COUNT == 8 {
        // exit from qemu
        debug::exit(debug::EXIT_SUCCESS);
    }

}

#[exception]
unsafe fn DefaultHandler(_irqn: i16){
    // irqn is error code 
}

// #[interrupt]
// fn TIM2(){
//    // 
// }