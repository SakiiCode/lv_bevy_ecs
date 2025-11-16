//! Auto-generated safe bindings to LVGL functions

use ::core::{
    ffi::{CStr, c_void},
    time::Duration,
};

use log::Level;

#[cfg(feature = "no_ecs")]
use crate::styles::Style;
use crate::{
    events::Event,
    subjects::Subject,
    widgets::{Wdg, Widget},
};

pub fn lv_init() {
    unsafe {
        lightvgl_sys::lv_init();
    }
}

pub fn lv_tick_inc(diff: Duration) {
    unsafe { lightvgl_sys::lv_tick_inc(diff.as_millis() as u32) }
}

pub fn lv_timer_handler() -> u32 {
    unsafe { lightvgl_sys::lv_timer_handler() }
}

#[rustfmt::skip]
pub fn lv_color_make(r: u8, g: u8, b: u8) -> lightvgl_sys::lv_color_t {
    crate::support::lv_color_make(r,g,b)
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

#[rustfmt::skip]
pub fn lv_subject_set_int(subject: &mut Subject, value: i32) {
    unsafe { lightvgl_sys::lv_subject_set_int(subject.raw_mut(), value) }
}

#[rustfmt::skip]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn lv_subject_set_string(subject: &mut Subject, value: *mut c_void) {
    unsafe { lightvgl_sys::lv_subject_set_pointer(subject.raw_mut(), value) }
}

#[rustfmt::skip]
pub fn lv_subject_set_color(subject: &mut Subject, value: lv_color_t) {
    unsafe { lightvgl_sys::lv_subject_set_color(subject.raw_mut(), value) }
}

pub fn lv_subject_get_int(subject: &mut Subject) -> i32 {
    unsafe { lightvgl_sys::lv_subject_get_int(subject.raw_mut()) }
}

pub fn lv_subject_get_ptr(subject: &mut Subject) -> *const c_void {
    unsafe { lightvgl_sys::lv_subject_get_pointer(subject.raw_mut()) }
}

pub fn lv_subject_get_color(subject: &mut Subject) -> lv_color_t {
    unsafe { lightvgl_sys::lv_subject_get_color(subject.raw_mut()) }
}

pub fn lv_subject_get_string(subject: &mut Subject) -> &CStr {
    unsafe { CStr::from_ptr(lightvgl_sys::lv_subject_get_string(subject.raw_mut())) }
}

pub fn lv_subject_get_previous_color(subject: &mut Subject) -> lv_color_t {
    unsafe { lightvgl_sys::lv_subject_get_previous_color(subject.raw_mut()) }
}

pub fn lv_subject_get_previous_int(subject: &mut Subject) -> i32 {
    unsafe { lightvgl_sys::lv_subject_get_previous_int(subject.raw_mut()) }
}

pub fn lv_subject_get_previous_string(subject: &mut Subject) -> &CStr {
    unsafe {
        CStr::from_ptr(lightvgl_sys::lv_subject_get_previous_string(
            subject.raw_mut(),
        ))
    }
}
pub fn lv_subject_get_previous_pointer(subject: &mut Subject) -> *const c_void {
    unsafe { lightvgl_sys::lv_subject_get_previous_pointer(subject.raw_mut()) }
}

#[rustfmt::skip]
pub fn lv_subject_add_observer_obj<'a, F>(subject: &'a mut Subject, object: &mut Widget, callback: F)
where
    F: FnMut(*mut lightvgl_sys::lv_observer_t, *mut lightvgl_sys::lv_subject_t) + 'a,
{
    crate::subjects::lv_subject_add_observer_obj(subject, object, callback)
}

pub fn lv_event_get_target(event: &mut lightvgl_sys::lv_event_t) -> *const c_void {
    unsafe { lightvgl_sys::lv_event_get_target(event) }
}

pub fn lv_event_get_target_obj(event: &mut lightvgl_sys::lv_event_t) -> Option<Wdg> {
    unsafe {
        let target = lightvgl_sys::lv_event_get_target_obj(event);
        Wdg::try_from_ptr(target)
    }
}

pub fn lv_event_get_current_target_obj(event: &mut lightvgl_sys::lv_event_t) -> Option<Wdg> {
    unsafe {
        let target = lightvgl_sys::lv_event_get_current_target_obj(event);
        Wdg::try_from_ptr(target)
    }
}

#[cfg(feature = "no_ecs")]
pub fn lv_obj_add_style(widget: &mut Wdg, mut style: Style, selector: lv_style_selector_t) {
    unsafe { lightvgl_sys::lv_obj_add_style(widget.raw_mut(), style.raw_mut(), selector) }
    core::mem::forget(style);
}

#[cfg(feature = "no_ecs")]
pub fn lv_obj_set_parent(obj: &mut Wdg, parent: &mut Wdg) {
    unsafe { lightvgl_sys::lv_obj_set_parent(obj.raw_mut(), parent.raw_mut()) }
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
