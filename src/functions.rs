//! Auto-generated safe bindings to LVGL functions

use std::time::Duration;

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

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
