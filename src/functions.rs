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

pub fn lv_color_make(r: u8, g: u8, b: u8) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_color_make(r, g, b) }
}

pub fn lv_log_add(
    level: crate::support::LogLevel,
    file: &cstr_core::CStr,
    line: u32,
    func: &cstr_core::CStr,
    message: &cstr_core::CStr,
) {
    unsafe {
        lightvgl_sys::lv_log_add(
            level.into(),
            file.as_ptr(),
            line as i32,
            func.as_ptr(),
            message.as_ptr(),
        );
    }
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
