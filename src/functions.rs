//! Auto-generated safe bindings to LVGL functions
#![allow(clippy::let_unit_value)]
#![allow(clippy::multiple_inherent_impl)]
#![allow(clippy::allow_attributes)]
#![allow(rustdoc::invalid_rust_codeblocks)]
#![allow(rustdoc::broken_intra_doc_links)]
use ::core::{ffi::CStr, num::NonZero};
use core::ffi::c_void;

use alloc::boxed::Box;

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

#[inline]
pub fn lv_init() {
    unsafe {
        lightvgl_sys::lv_init();
    }
}

#[inline]
pub fn lv_tick_inc(millis: u32) {
    unsafe { lightvgl_sys::lv_tick_inc(millis) }
}

// this is horrible but lv_tick_set_cb does not accept user data
// could be replaced with StaticCell or OnceCell but not worth the extra dependency
static mut TICK_CALLBACK: Option<Box<dyn FnMut() -> u32>> = None;

pub fn lv_tick_set_cb<F>(callback: F)
where
    F: FnMut() -> u32 + 'static,
{
    unsafe {
        TICK_CALLBACK = Some(Box::new(callback));
        lightvgl_sys::lv_tick_set_cb(Some(tick_callback_trampoline));
    }
}

#[expect(static_mut_refs)]
unsafe extern "C" fn tick_callback_trampoline() -> u32 {
    unsafe { (*TICK_CALLBACK.as_mut().unwrap())() }
}

#[inline]
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

#[inline]
pub fn lv_pct(pct: lightvgl_sys::lv_coord_t) -> lightvgl_sys::lv_coord_t {
    unsafe { lightvgl_sys::lv_pct(pct) }
}

#[inline]
pub fn lv_dpx(n: i32) -> i32 {
    unsafe { lightvgl_sys::lv_dpx(n) }
}

#[cfg(LV_USE_GRID)]
#[inline]
pub fn lv_grid_fr(x: u8) -> i32 {
    unsafe { lightvgl_sys::lv_grid_fr(x) }
}

#[inline]
pub fn lv_color_make(r: u8, g: u8, b: u8) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_color_make(r, g, b) }
}

#[inline]
pub fn lv_color_hex(c: u32) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_color_hex(c) }
}

#[inline]
pub fn lv_color_hex3(c: u32) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_color_hex3(c) }
}

#[inline]
pub fn lv_color_mix(c1: lv_color_t, c2: lv_color_t, mix: u8) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_color_mix(c1, c2, mix) }
}

#[inline]
pub fn lv_palette_darken(p: lv_palette_t::Type, lvl: u8) -> lightvgl_sys::lv_color_t {
    unsafe { lightvgl_sys::lv_palette_darken(p, lvl) }
}

#[cfg(LV_USE_LOG)]
#[inline]
pub fn lv_log_add(level: log::Level, file: &CStr, line: u32, func: &CStr, message: &CStr) {
    crate::logging::lv_log_add(level, file, line, func, message)
}

#[inline]
pub fn lv_async_call<F>(callback: F)
where
    F: FnMut() + 'static,
{
    unsafe {
        lightvgl_sys::lv_async_call(
            Some(async_call_trampoline::<F>),
            Box::into_raw(Box::new(callback)).cast(),
        );
    }
}

unsafe extern "C" fn async_call_trampoline<F>(obj: *mut c_void)
where
    F: FnMut() + 'static,
{
    unsafe {
        if !obj.is_null() {
            let callback = &mut *(obj.cast::<F>());
            callback();
        } else {
            crate::warn!("Async call object was null, this should never happen!");
        }
    }
}

#[inline]
pub fn lv_screen_active() -> Option<Wdg> {
    unsafe { Wdg::try_from_ptr(lightvgl_sys::lv_screen_active()) }
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
