//! Auto-generated safe bindings to LVGL functions

use std::time::Duration;

use log::Level;

use crate::{events::Event, widgets::Wdg};

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

#[rustfmt::skip]
pub fn lv_color_make(r: u8, g: u8, b: u8) -> lightvgl_sys::lv_color_t {
    crate::support::lv_color_make(r,g,b)
}

#[rustfmt::skip]
pub fn lv_log_init() {
    crate::logging::lv_log_init();
}

#[rustfmt::skip]
pub fn lv_log_add(level: Level, file: &core::ffi::CStr, line: u32, func: &core::ffi::CStr, message: &core::ffi::CStr) {
    crate::logging::lv_log_add(level, file, line, func, message)
}

#[rustfmt::skip]
pub fn lv_obj_add_event_cb<'a, F>(widget: &'a mut Wdg, filter: Event, callback: F)
where
    F: FnMut(lightvgl_sys::lv_event_t) + 'a,
{
    crate::events::lv_obj_add_event_cb(widget, filter, callback)
}

#[rustfmt::skip]
pub fn lv_async_call<F>(callback: F)
where
    F: FnMut() + 'static
{
    crate::timers::lv_async_call(callback)
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
