#![cfg(not(test))]

use super::bindings::logz_rs_error_handler;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    // Log is probably too long to print
    unsafe {
        logz_rs_error_handler();
    }
    unreachable!();
}
