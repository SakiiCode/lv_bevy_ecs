use std::{
    ffi::{CStr, c_char, c_void},
    mem::MaybeUninit,
};

use lvgl_sys::{lv_color_t, lv_subject_t};

use crate::widgets::Widget;

pub struct Subject {
    raw: lv_subject_t,
}

unsafe impl Send for Subject {}
unsafe impl Sync for Subject {}

impl Subject {
    pub fn new_int(value: i32) -> Self {
        unsafe {
            let mut subject = MaybeUninit::<lvgl_sys::lv_subject_t>::uninit();
            lvgl_sys::lv_subject_init_int(subject.as_mut_ptr(), value);
            Self {
                raw: subject.assume_init(),
            }
        }
    }

    pub fn new_string(value: &CStr) -> Self {
        unsafe {
            let mut subject = MaybeUninit::<lvgl_sys::lv_subject_t>::uninit();
            let len = value.count_bytes();
            let zero: c_char = 0;
            lvgl_sys::lv_subject_init_string(
                subject.as_mut_ptr(),
                &mut Box::leak(vec![zero; len].into_boxed_slice())[0],
                std::ptr::null_mut(),
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
            let mut subject = MaybeUninit::<lvgl_sys::lv_subject_t>::uninit();
            lvgl_sys::lv_subject_init_pointer(subject.as_mut_ptr(), value);
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
        lvgl_sys::lv_subject_set_int(subject.raw(), value);
    }
}

pub fn lv_subject_set_string(subject: &mut Subject, value: *mut c_void) {
    unsafe {
        lvgl_sys::lv_subject_set_pointer(subject.raw(), value);
    }
}

pub fn lv_subject_set_color(subject: &mut Subject, value: lv_color_t) {
    unsafe {
        lvgl_sys::lv_subject_set_color(subject.raw(), value);
    }
}

pub fn lv_subject_get_int(subject: &mut Subject) -> i32 {
    unsafe { lvgl_sys::lv_subject_get_int(subject.raw()) }
}

pub fn lv_subject_get_ptr(subject: &mut Subject) -> *const c_void {
    unsafe { lvgl_sys::lv_subject_get_pointer(subject.raw()) }
}

pub fn lv_subject_get_color(subject: &mut Subject) -> lv_color_t {
    unsafe { lvgl_sys::lv_subject_get_color(subject.raw()) }
}

pub fn lv_subject_get_string(subject: &mut Subject) -> &CStr {
    unsafe { CStr::from_ptr(lvgl_sys::lv_subject_get_string(subject.raw())) }
}

pub fn lv_subject_get_previous_color(subject: &mut Subject) -> lv_color_t {
    unsafe { lvgl_sys::lv_subject_get_previous_color(subject.raw()) }
}

pub fn lv_subject_get_previous_int(subject: &mut Subject) -> i32 {
    unsafe { lvgl_sys::lv_subject_get_previous_int(subject.raw()) }
}

pub fn lv_subject_get_previous_string(subject: &mut Subject) -> &CStr {
    unsafe { CStr::from_ptr(lvgl_sys::lv_subject_get_previous_string(subject.raw())) }
}
pub fn lv_subject_get_previous_pointer(subject: &mut Subject) -> *const c_void {
    unsafe { lvgl_sys::lv_subject_get_previous_pointer(subject.raw()) }
}

pub fn lv_subject_add_observer_obj<'a, F>(subject: &'a mut Subject, object: &mut Widget, callback: F)
where
    F: FnMut(*mut lvgl_sys::lv_observer_t, *mut lvgl_sys::lv_subject_t) + 'a,
{
    unsafe {
        lvgl_sys::lv_subject_add_observer_obj(
            &mut subject.raw,
            Some(subject_callback::<F>),
            object.raw(),
            Box::into_raw(Box::new(callback)) as *mut c_void,
        );
    }
}

pub(crate) unsafe extern "C" fn subject_callback<F>(
    observer: *mut lvgl_sys::lv_observer_t,
    subject: *mut lvgl_sys::lv_subject_t,
) where
    F: FnMut(*mut lvgl_sys::lv_observer_t, *mut lvgl_sys::lv_subject_t),
{
    unsafe {
        let callback = &mut *((*observer).user_data as *mut F);
        callback(observer, subject);
    }
}
