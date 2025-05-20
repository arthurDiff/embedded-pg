//Hardware Abstraction Layer (HAL)
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::fmt::Write;
use tm4c123x_hal::serial::{NewlineMode, Serial};
use tm4c123x_hal::{self as hal, sysctl};
use tm4c123x_hal::prelude::*;

#[entry]
fn main() -> !{
    let p = hal::Peripherals::take().unwrap();
    let _cp = hal::CorePeripherals::take().unwrap();

    //wrap up the sysctl struct into an obj with higher-layer api
    let mut sc = p.SYSCTL.constrain();
    // Pick our scillation settings
    sc.clock_setup.oscillator = sysctl::Oscillator::Main(sysctl::CrystalFrequency::_16mhz, sysctl::SystemClock::UsePll(sysctl::PllOutputFrequency::_80_00mhz));

    // Configure the PLL with those settings
    let clocks = sc.clock_setup.freeze();

    // Wrap up the GPIO_PORTA struct into an object with a higher-layer API
    // NOTE: it needs to borrow `sc.power_control` so it can power up the GPIO
    // peripheral automatically.
    let mut porta =p.GPIO_PORTA.split(&sc.power_control);

    // Activate the UART.
    let mut uart = Serial::uart0(p.UART0,
    // The transmit pin
    porta.pa1.into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    // the receive pin
    porta.pa0.into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    // No RTS or CTS requried,
    (),
    (),
    // the baud rate
    115200_u32.bps(),
    // Output handling
    NewlineMode::SwapLFtoCRLF,
    // we need tohe clock rates to calculate the baud rate divisor
    &clocks,
    // We need this to power up the UART peripheral
    &sc.power_control 
    );

    loop{
        writeln!(uart, "Hello, Worda!").unwrap()
    }
}