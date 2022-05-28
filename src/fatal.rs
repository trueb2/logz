#![cfg(not(test))]

use super::bindings::zlog_rs_error_handler;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // Log is probably too long to print
    unsafe {
        zlog_rs_error_handler();
    }
    unreachable!();
}
