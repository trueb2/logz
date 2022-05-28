#![cfg_attr(not(test), no_std, no_main)]
#![cfg_attr(test, allow(unused))]

#[cfg(test)]
#[macro_use]
extern crate std;
#[cfg(test)]
use std::prelude::*;

use core::fmt::Write;
use heapless::String;
use log::*;

mod bindings;
use bindings::*;
pub mod fatal;

pub static LOGZ_LOGGER: ZLog = ZLog;

pub struct ZLog;

fn logz_init(lvl: LevelFilter) {
    log::set_logger(&LOGZ_LOGGER).unwrap();
    log::set_max_level(lvl);
    log::info!("Initialized logz")
}

#[no_mangle]
pub extern "C" fn logz_init_error() {
    logz_init(LevelFilter::Error);
}
#[no_mangle]
pub extern "C" fn logz_init_warn() {
    logz_init(LevelFilter::Warn);
}
#[no_mangle]
pub extern "C" fn logz_init_info() {
    logz_init(LevelFilter::Info);
}
#[no_mangle]
pub extern "C" fn logz_init_trace() {
    logz_init(LevelFilter::Trace);
}

impl Log for ZLog {
    #[inline(always)]
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    #[inline(always)]
    fn log(&self, record: &Record) {
        let log_impl = match record.level() {
            Level::Error => log_err,
            Level::Warn => log_wrn,
            _ => log_inf,
        };
        let mut c_str = String::<256>::new();
        write!(c_str, "{}: {}\0", record.target(), record.args()).unwrap();
        unsafe {
            log_impl(c_str.as_bytes().as_ptr() as *const u8 as *const cty::c_char);
        }
    }

    #[inline(always)]
    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
