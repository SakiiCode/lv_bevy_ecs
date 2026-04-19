//! Auto-generated safe bindings to LVGL functions

use ::core::{
    ffi::{CStr, c_void},
    num::NonZero,
    time::Duration,
};

use crate::styles::Style;
use crate::widgets::Wdg;

pub enum NextTimerPeriod {
    /// At least one timer is waiting to be polled
    Ready,
    /// LVGL can wait this many milliseconds
    AfterMs(NonZero<u32>),
    /// There is no timer running
    Never,
}

pub fn lv_init() {
    unsafe {
        lightvgl_sys::lv_init();
    }
}

pub fn lv_tick_inc(diff: Duration) {
    unsafe { lightvgl_sys::lv_tick_inc(diff.as_millis() as u32) }
}

pub fn lv_tick_set_cb<F>(callback: F)
where
    F: FnMut() -> u32 + 'static,
{
    crate::timers::lv_tick_set_cb(callback);
}

pub fn lv_timer_handler() -> NextTimerPeriod {
    unsafe {
        let next_timer_ms = lightvgl_sys::lv_timer_handler();
        match next_timer_ms {
            0 => NextTimerPeriod::Ready,
            LV_NO_TIMER_READY => NextTimerPeriod::Never,
            _ => NextTimerPeriod::AfterMs(NonZero::new_unchecked(next_timer_ms)),
        }
    }
}

pub fn lv_pct(pct: lightvgl_sys::lv_coord_t) -> lightvgl_sys::lv_coord_t {
    unsafe { lightvgl_sys::lv_pct(pct) }
}

pub fn lv_dpx(n: i32) -> i32 {
    unsafe { lightvgl_sys::lv_dpx(n) }
}

#[cfg(LV_USE_GRID)]
pub fn lv_grid_fr(x: u8) -> i32 {
    unsafe { lightvgl_sys::lv_grid_fr(x) }
}

pub fn lv_color_make(r: u8, g: u8, b: u8) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_color_make(r, g, b) }
}

pub fn lv_color_hex(c: u32) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_color_hex(c) }
}

pub fn lv_color_hex3(c: u32) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_color_hex3(c) }
}

pub fn lv_color_mix(c1: lv_color_t, c2: lv_color_t, mix: u8) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_color_mix(c1, c2, mix) }
}

pub fn lv_palette_darken(p: lv_palette_t, lvl: u8) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_palette_darken(p, lvl) }
}

#[cfg(LV_USE_LOG)]
pub fn lv_log_add(level: log::Level, file: &CStr, line: u32, func: &CStr, message: &CStr) {
    crate::logging::lv_log_add(level, file, line, func, message)
}

pub fn lv_async_call<F>(callback: F)
where
    F: FnMut() + 'static,
{
    crate::timers::lv_async_call(callback)
}

pub fn lv_screen_active() -> Option<Wdg> {
    unsafe { Wdg::try_from_ptr(lightvgl_sys::lv_screen_active()) }
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
