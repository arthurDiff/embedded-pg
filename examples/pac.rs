// Peripheral Access Crate (PAC)
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use tm4c123x;
use panic_halt as _;

#[entry]
fn init() -> ! {
    let _cp =cortex_m::Peripherals::take().unwrap();
    let p = tm4c123x::Peripherals::take().unwrap();

    let pwm = p.PWM0;
    pwm.ctl.write(|w| w.globalsync0().clear_bit());
    // Mode = 1 => Count up/down mode
    pwm._2_ctl.write(|w|w.enable().set_bit().mode().set_bit());
    pwm._2_gena.write(|w| w.actcmpau().zero().actcmpad().one());
    // 528 cycles (264 up and down) = 4 loops per video line (2112 cycles)
    pwm._2_load.write(|w|unsafe{w.load().bits(263)});
    pwm._2_cmpa.write(|w|unsafe{w.compa().bits(64)});
    pwm.enable.write(|w| w.pwm4en().set_bit());

    // Read example
    if pwm.ctl.read().globalsync0().bit_is_set(){
        // do something
    }
    
    // Write example
    pwm.ctl.write(|w| w.globalsync0().clear_bit());

    // Modify example
    pwm.ctl.modify(|_r, w| w.globalsync0().clear_bit());
    
    loop{}
}