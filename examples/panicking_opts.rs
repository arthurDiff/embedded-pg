#![no_main]
#![no_std]

// to semi host to host stdout
// use panic_semihosting as _;

// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
use panic_halt as _;

// // release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
use panic_abort as _;

use cortex_m_rt::entry;


#[entry]
fn main() -> !{
    let t = [1,2,3,4];
    let len = t.len();
    let _failed = t[len];

    loop{}
}