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
mod fatal;

pub static LOGGER: ZLog = ZLog;

pub struct ZLog;

fn zlog_init(lvl: LevelFilter) {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(lvl);
    log::info!("Initialized zlog")
}

#[no_mangle]
pub extern "C" fn zlog_init_error() {
    zlog_init(LevelFilter::Error);
}
#[no_mangle]
pub extern "C" fn zlog_init_warn() {
    zlog_init(LevelFilter::Warn);
}
#[no_mangle]
pub extern "C" fn zlog_init_info() {
    zlog_init(LevelFilter::Info);
}
#[no_mangle]
pub extern "C" fn zlog_init_trace() {
    zlog_init(LevelFilter::Trace);
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
