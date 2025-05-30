#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;

use alloc::vec::Vec;
use cortex_m::{asm, interrupt};
use cortex_m_rt::entry;
use panic_semihosting as _;

// Using bump allocator to GlobalAlloc

struct BumpPointerAlloc {
    head: UnsafeCell<usize>,
    end: usize,
}

unsafe impl Sync for BumpPointerAlloc {}

unsafe impl GlobalAlloc for BumpPointerAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        interrupt::free(|_| {
            let head = self.head.get();
            let size = layout.size();
            let align = layout.align();
            // bit wise not operator
            let align_mask = !(align - 1);

            // movs start up to next alignment boundary
            let start = (unsafe { *head } + align - 1) & align_mask;

            if start + size > self.end {
                // a null pointer to signal an Out Of Memory condition
                core::ptr::null_mut()
            } else {
                unsafe {
                    *head = start + size;
                }
                start as *mut u8
            }
        })
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // never deallocate
    }
}

// Declaration of the global memory allocator
// NOTE the user must ensure that the memory region `[0x2000_0100, 0x2000_0200]`
// is not used by other parts of the program
#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc {
    head: UnsafeCell::new(0x2000_0100),
    end: 0x2000_0200,
};

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}

#[entry]
fn main() -> ! {
    let mut xs = Vec::new();

    xs.push(42);

    assert_eq!(xs.pop(), Some(42));

    loop {}
}
