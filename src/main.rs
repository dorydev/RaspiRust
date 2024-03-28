#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

//#![no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe
    {
        loop {
            core::ptr::write_volatile(dst, src)
        }
    }
}
