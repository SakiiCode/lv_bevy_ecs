//! # Subjects
//!
//! ```rust
//! let mut chart_type_subject = Subject::new_int(0);
//!
//! unsafe {
//!     lv_dropdown_bind_value(dropdown.raw(), chart_type_subject.raw());
//! }
//!
//! lv_subject_add_observer_obj(&mut chart_type_subject, &mut chart, |observer, subject|{
//!     // ...
//! });
//! lv_subject_set_int(&mut chart_type_subject, 1);
//!
//! ```

use core::{
    ffi::{CStr, c_char, c_void},
    mem::MaybeUninit,
};

use alloc::{boxed::Box, vec};
use bevy_ecs::component::Component;
use lightvgl_sys::{lv_color_t, lv_subject_t};

use crate::{trace, widgets::Widget};

#[derive(Component)]
pub struct Subject {
    raw: lv_subject_t,
}

impl Drop for Subject {
    fn drop(&mut self) {
        unsafe {
            lightvgl_sys::lv_subject_deinit(&mut self.raw);
        }
    }
}

unsafe impl Send for Subject {}
unsafe impl Sync for Subject {}

impl Subject {
    pub fn new_int(value: i32) -> Self {
        unsafe {
            let mut subject = MaybeUninit::<lightvgl_sys::lv_subject_t>::uninit();
            lightvgl_sys::lv_subject_init_int(subject.as_mut_ptr(), value);
            Self {
                raw: subject.assume_init(),
            }
        }
    }

    pub fn new_string(value: &CStr) -> Self {
        unsafe {
            let mut subject = MaybeUninit::<lightvgl_sys::lv_subject_t>::uninit();
            let len = value.count_bytes();
            let zero: c_char = 0;
            lightvgl_sys::lv_subject_init_string(
                subject.as_mut_ptr(),
                &mut Box::leak(vec![zero; len].into_boxed_slice())[0],
                core::ptr::null_mut(),
                len,
                value.as_ptr(),
            );
            Self {
                raw: subject.assume_init(),
            }
        }
    }

    pub fn new_ptr(value: *mut c_void) -> Self {
        unsafe {
            let mut subject = MaybeUninit::<lightvgl_sys::lv_subject_t>::uninit();
            lightvgl_sys::lv_subject_init_pointer(subject.as_mut_ptr(), value);
            Self {
                raw: subject.assume_init(),
            }
        }
    }

    pub fn raw(&mut self) -> &mut lv_subject_t {
        &mut self.raw
    }
}

pub fn lv_subject_set_int(subject: &mut Subject, value: i32) {
    unsafe {
        lightvgl_sys::lv_subject_set_int(subject.raw(), value);
    }
}

pub fn lv_subject_set_string(subject: &mut Subject, value: *mut c_void) {
    unsafe {
        lightvgl_sys::lv_subject_set_pointer(subject.raw(), value);
    }
}

pub fn lv_subject_set_color(subject: &mut Subject, value: lv_color_t) {
    unsafe {
        lightvgl_sys::lv_subject_set_color(subject.raw(), value);
    }
}

pub fn lv_subject_get_int(subject: &mut Subject) -> i32 {
    unsafe { lightvgl_sys::lv_subject_get_int(subject.raw()) }
}

pub fn lv_subject_get_ptr(subject: &mut Subject) -> *const c_void {
    unsafe { lightvgl_sys::lv_subject_get_pointer(subject.raw()) }
}

pub fn lv_subject_get_color(subject: &mut Subject) -> lv_color_t {
    unsafe { lightvgl_sys::lv_subject_get_color(subject.raw()) }
}

pub fn lv_subject_get_string(subject: &mut Subject) -> &CStr {
    unsafe { CStr::from_ptr(lightvgl_sys::lv_subject_get_string(subject.raw())) }
}

pub fn lv_subject_get_previous_color(subject: &mut Subject) -> lv_color_t {
    unsafe { lightvgl_sys::lv_subject_get_previous_color(subject.raw()) }
}

pub fn lv_subject_get_previous_int(subject: &mut Subject) -> i32 {
    unsafe { lightvgl_sys::lv_subject_get_previous_int(subject.raw()) }
}

pub fn lv_subject_get_previous_string(subject: &mut Subject) -> &CStr {
    unsafe { CStr::from_ptr(lightvgl_sys::lv_subject_get_previous_string(subject.raw())) }
}
pub fn lv_subject_get_previous_pointer(subject: &mut Subject) -> *const c_void {
    unsafe { lightvgl_sys::lv_subject_get_previous_pointer(subject.raw()) }
}

pub fn lv_subject_add_observer_obj<'a, F>(
    subject: &'a mut Subject,
    object: &mut Widget,
    callback: F,
) where
    F: FnMut(*mut lightvgl_sys::lv_observer_t, *mut lightvgl_sys::lv_subject_t) + 'a,
{
    trace!("lv_subject_add_observer_obj");
    unsafe {
        lightvgl_sys::lv_subject_add_observer_obj(
            &mut subject.raw,
            Some(subject_callback::<F>),
            object.raw(),
            Box::into_raw(Box::new(callback)) as *mut c_void,
        );
    }
}

pub(crate) unsafe extern "C" fn subject_callback<F>(
    observer: *mut lightvgl_sys::lv_observer_t,
    subject: *mut lightvgl_sys::lv_subject_t,
) where
    F: FnMut(*mut lightvgl_sys::lv_observer_t, *mut lightvgl_sys::lv_subject_t),
{
    unsafe {
        let callback = &mut *((*observer).user_data as *mut F);
        callback(observer, subject);
    }
}
