#![no_main]
#![no_std]

use volatile_register::{RO, RW};

// #[repr(C)]
// struct SysTick {
//     // Control and Status Register | SYST_CSR
//     pub csr: RW<u32>,
//     // Reload Value Register | SYST_RVR
//     pub rvr: RW<u32>,
//     // Current Value Register | SYST_CVR
//     pub cvr: RW<u32>,
//     // Calibration Value Register | SYST_CALIB
//     pub calib: RO<u32>,
// }

// fn get_systick() -> &'static mut SysTick {
//     unsafe { &mut *(0xE000_E010 as *mut SysTick) }
// }

// fn get_time() -> u32 {
//     let systick = get_systick();
//     systick.cvr.read()
// }

#[repr(C)]
struct RegisterBlock {
    pub csr: RW<u32>,
    pub rvr: RW<u32>,
    pub cvr: RW<u32>,
    pub calib: RO<u32>,
}
pub struct SystemTimer {
    p: &'static mut RegisterBlock,
}

impl SystemTimer {
    pub fn new() -> SystemTimer {
        Self {
            p: unsafe { &mut *(0xE000_E010 as *mut RegisterBlock) },
        }
    }

    pub fn get_time(&self) -> u32 {
        self.p.cvr.read()
    }

    pub fn set_reload(&mut self, reload_value: u32) {
        unsafe { self.p.rvr.write(reload_value) }
    }
}

pub fn example_use() -> u32 {
    let mut s = SystemTimer::new();
    s.set_reload(0x00FF_FFFF);
    // format!("Time is now 0x{:08x}", s.get_time())
    s.get_time()
}
