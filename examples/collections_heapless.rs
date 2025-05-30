#![no_std]
#![no_main]

use cortex_m_rt::entry;
use heapless::Vec;

#[entry]
fn main() -> ! {
    let mut x: Vec<_, 255> = Vec::new();

    x.push(42).unwrap();

    assert_eq!(x.pop(), Some(42));
    loop {}
}
