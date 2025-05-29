#![no_main]
#![no_std]

use core::cell::{Cell, RefCell, UnsafeCell};

use cortex_m::interrupt::{self, Mutex};
use cortex_m_rt::entry;
use stm32f4::stm32f405;

struct CSCounter(UnsafeCell<usize>);

const CS_COUNTER_INIT: CSCounter = CSCounter(UnsafeCell::new(0));

impl CSCounter {
    pub fn reset(&self, _cs: &interrupt::CriticalSection) {
        unsafe { *self.0.get() = 0 };
    }

    pub fn increment(&self, _cs: &interrupt::CriticalSection) {
        unsafe { *self.0.get() += 1 };
    }
}

unsafe impl Sync for CSCounter {}

static COUNTER: CSCounter = CS_COUNTER_INIT;
static COUNTER2: Mutex<Cell<u32>> = Mutex::new(Cell::new(0));

static MY_GPIO: Mutex<RefCell<Option<stm32f405::GPIOA>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = stm32f405::Peripherals::take().unwrap();
    let gpio = &dp.GPIOA;

    // example config step
    // configure_gpio(gpioa);

    interrupt::free(|cs| MY_GPIO.borrow(cs).replace(Some(dp.GPIOA)));

    // enable interrupt
    // set_timer_1hz();

    let mut last_state = false;
    loop {
        let state = interrupt::free(|cs| {
            let gpioa = MY_GPIO.borrow(cs).borrow();
            gpioa.as_ref().unwrap().idr().read().idr0().bit_is_set()
        });
        if state && !last_state {
            interrupt::free(|cs| {
                let gpioa = MY_GPIO.borrow(cs).borrow();
                gpioa
                    .as_ref()
                    .unwrap()
                    .odr()
                    .modify(|_, w| w.odr1().set_bit());
            })
        }
        last_state = state;
    }
}

// #[interrupt]
fn timers() {
    interrupt::free(|cs| {
        let gpioa = MY_GPIO.borrow(cs).borrow();
        gpioa
            .as_ref()
            .unwrap()
            .odr()
            .modify(|_, w| w.odr1().clear_bit());
    })
}
