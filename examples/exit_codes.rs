#![no_main]
#![no_std]

// use cortex_m_rt::entry;
// use cortex_m_semihosting::debug;
// use panic_halt as _;

// #[entry]
// fn main() -> !{
//     let roses = "blue";

//     if roses == "blue"{
//         debug::exit(debug::EXIT_SUCCESS);
//     }else {
//         debug::exit(debug::EXIT_FAILURE);
//     }

//     loop{}
// }

use panic_semihosting as _;

#[allow(unused_imports)]
use cortex_m_semihosting::debug;
use cortex_m_rt::entry;

#[entry]
fn main()->! {
    let pie = "pizza";

    assert_eq!(pie, "blueberry"); 
    
    loop{}
}