//! Auto-generated safe bindings to LVGL functions

use core::time::Duration;

use crate::{error, logging::LvglLogger};
use alloc::string::ToString;

pub fn lv_init() {
    unsafe {
        lightvgl_sys::lv_init();
    }
}

pub fn lv_tick_inc(diff: Duration) {
    unsafe {
        lightvgl_sys::lv_tick_inc(diff.as_millis() as u32);
    }
}

pub fn lv_timer_handler() {
    unsafe {
        lightvgl_sys::lv_timer_handler();
    }
}

pub fn lv_color_make(r: u8, g: u8, b: u8) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_color_make(r, g, b) }
}

pub fn lv_log_init() {
    match log::set_logger(&LvglLogger) {
        Ok(_) => {
            log::set_max_level(log::LevelFilter::Trace);
        }
        Err(err) => {
            error!("Could not initialize logging: {}", err.to_string());
        }
    };
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
