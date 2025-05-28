#![no_std]

// MANUAL IMPL
// use core::mem::replace;

// // simulating serial port
// type SerialPort = u8;

// struct Peripherals {
//     serial: Option<SerialPort>,
// }

// impl Peripherals {
//     fn take_serial(&mut self) -> SerialPort {
//         let p = replace(&mut self.serial, None);
//         p.unwrap()
//     }
// }

// static mut PERIPHERALS: Peripherals = Peripherals { serial: Some(100) };

use cortex_m::singleton;

fn main() {
    let x: &'static mut bool = singleton!(: bool = false).unwrap();
}
